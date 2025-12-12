use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        client::Client,
        endpoint::{Endpoint, EndpointError},
        object::{Object, ObjectPrivate},
        poll::{self, PollError, PollEvent, Poller},
        protocols::wayland::wl_display::WlDisplay,
        trans::{FlushResult, TransError},
        utils::{
            env::{WAYLAND_DISPLAY, WAYLAND_SOCKET, XDG_RUNTIME_DIR},
            handler_holder::HandlerHolder,
            stack::Stack,
            stash::Stash,
        },
    },
    error_reporter::Report,
    isnt::std_1::primitive::IsntStrExt,
    run_on_drop::on_drop,
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        env::{remove_var, var, var_os},
        fmt,
        io::{self, BufWriter, Write, pipe},
        os::{
            fd::{FromRawFd, OwnedFd},
            unix::ffi::OsStrExt,
        },
        rc::{Rc, Weak},
        str::FromStr,
        sync::{
            Arc,
            atomic::{
                AtomicBool,
                Ordering::{Acquire, Release},
            },
        },
        time::Duration,
    },
    thiserror::Error,
    uapi::{
        Fd,
        c::{self, sockaddr_un},
    },
};

#[derive(Debug, Error)]
#[error(transparent)]
pub struct StateError(#[from] StateErrorKind);

#[derive(Debug, Error)]
enum StateErrorKind {
    #[error("the state has already been destroyed")]
    Destroyed,
    #[error("the state has been destroyed by a remote destructor")]
    RemoteDestroyed,
    #[error("cannot perform recursive call into the state")]
    RecursiveCall,
    #[error("the server hung up the connection")]
    ServerHangup,
    #[error("could not write to the server socket")]
    WriteToServer(#[source] EndpointError),
    #[error("could not dispatch server events")]
    DispatchEvents(#[source] EndpointError),
    #[error("could not create a socket pair")]
    Socketpair(#[source] io::Error),
    #[error(transparent)]
    CreateAcceptor(AcceptorError),
    #[error("could not accept a new connection")]
    AcceptConnection(AcceptorError),
    #[error("could not create a pipe")]
    CreatePipe(#[source] io::Error),
    #[error("could not read {} environment variable", WAYLAND_DISPLAY)]
    WaylandDisplay,
    #[error("{} is not set", XDG_RUNTIME_DIR)]
    XrdNotSet,
    #[error("the socket path is too long")]
    SocketPathTooLong,
    #[error("could not create a socket")]
    CreateSocket(#[source] io::Error),
    #[error("could not connect to {0}")]
    Connect(String, #[source] io::Error),
    #[error("{} does not contain a valid number", WAYLAND_SOCKET)]
    WaylandSocketNotNumber,
    #[error("F_GETFD failed on {}", WAYLAND_SOCKET)]
    WaylandSocketGetFd(#[source] io::Error),
    #[error("F_SETFD failed on {}", WAYLAND_SOCKET)]
    WaylandSocketSetFd(#[source] io::Error),
    #[error(transparent)]
    PollError(PollError),
}

pub(crate) enum Pollable {
    Endpoint(EndpointWithClient),
    Acceptor(Rc<Acceptor>),
    Destructor(#[expect(dead_code)] OwnedFd, Arc<AtomicBool>),
}

#[derive(Clone)]
pub(crate) struct EndpointWithClient {
    endpoint: Rc<Endpoint>,
    client: Option<Rc<Client>>,
}

pub struct State {
    poller: Poller,
    next_pollable_id: Cell<u64>,
    pub(crate) server: Rc<Endpoint>,
    pub(crate) destroyed: Cell<bool>,
    handler: HandlerHolder<dyn StateHandler>,
    pub(crate) pollables: RefCell<HashMap<u64, Pollable>>,
    acceptable_acceptors: Stack<Rc<Acceptor>>,
    has_acceptable_acceptors: Cell<bool>,
    clients_to_kill: Stack<Rc<Client>>,
    has_clients_to_kill: Cell<bool>,
    readable_endpoints: Stack<EndpointWithClient>,
    has_readable_endpoints: Cell<bool>,
    flushable_endpoints: Stack<EndpointWithClient>,
    has_flushable_endpoints: Cell<bool>,
    interest_update_endpoints: Stack<Rc<Endpoint>>,
    has_interest_update_endpoints: Cell<bool>,
    interest_update_acceptors: Stack<Rc<Acceptor>>,
    has_interest_update_acceptors: Cell<bool>,
    pub(crate) all_proxies: RefCell<HashMap<u64, Weak<dyn Object>>>,
    pub(crate) next_proxy_id: Cell<u64>,
    pub(crate) log: bool,
    pub(crate) log_prefix: String,
    log_writer: RefCell<BufWriter<Fd>>,
    global_lock_held: Cell<bool>,
    pub(crate) proxy_stash: Stash<Rc<dyn Object>>,
}

pub struct StateBuilder {
    server_fd: Option<Rc<OwnedFd>>,
    log: bool,
    log_prefix: String,
}

impl Default for StateBuilder {
    fn default() -> Self {
        Self {
            server_fd: None,
            log: var("WL_PROXY_DEBUG").as_deref() == Ok("1"),
            log_prefix: "".to_string(),
        }
    }
}

impl StateBuilder {
    pub fn build(self) -> Result<Rc<State>, StateError> {
        let server_fd = match self.server_fd {
            Some(fd) => fd,
            _ => 'fd: {
                if let Some(wayland_socket) = var_os(WAYLAND_SOCKET) {
                    let fd = str::from_utf8(wayland_socket.as_bytes())
                        .ok()
                        .and_then(|s| i32::from_str(s).ok())
                        .ok_or(StateErrorKind::WaylandSocketNotNumber)?;
                    let flags = uapi::fcntl_getfd(fd)
                        .map_err(|e| StateErrorKind::WaylandSocketGetFd(e.into()))?;
                    uapi::fcntl_setfd(fd, flags | c::FD_CLOEXEC)
                        .map_err(|e| StateErrorKind::WaylandSocketSetFd(e.into()))?;
                    unsafe {
                        remove_var(WAYLAND_SOCKET);
                    }
                    break 'fd unsafe { Rc::new(OwnedFd::from_raw_fd(fd)) };
                }
                let mut name = var(WAYLAND_DISPLAY)
                    .ok()
                    .ok_or(StateErrorKind::WaylandDisplay)?;
                if name.is_empty() {
                    return Err(StateErrorKind::WaylandDisplay.into());
                }
                if !name.starts_with("/") {
                    let Ok(xrd) = var(XDG_RUNTIME_DIR) else {
                        return Err(StateErrorKind::XrdNotSet.into());
                    };
                    name = format!("{xrd}/{name}");
                }
                let mut addr = sockaddr_un {
                    sun_family: c::AF_UNIX as _,
                    sun_path: [0; 108],
                };
                if name.len() > addr.sun_path.len() - 1 {
                    return Err(StateErrorKind::SocketPathTooLong.into());
                }
                let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
                sun_path[..name.len()].copy_from_slice(name.as_bytes());
                sun_path[name.len()] = 0;
                let socket = uapi::socket(c::AF_UNIX, c::SOCK_STREAM | c::SOCK_CLOEXEC, 0)
                    .map_err(|e| StateErrorKind::CreateSocket(e.into()))?;
                uapi::connect(socket.raw(), &addr)
                    .map_err(|e| StateErrorKind::Connect(name.to_string(), e.into()))?;
                Rc::new(socket.into())
            }
        };
        const SERVER_ENDPOINT_ID: u64 = 0;
        let server = Endpoint::new(SERVER_ENDPOINT_ID, &server_fd);
        server.idl.acquire();
        server.idl.acquire();
        let mut endpoints = HashMap::new();
        endpoints.insert(
            SERVER_ENDPOINT_ID,
            Pollable::Endpoint(EndpointWithClient {
                endpoint: server.clone(),
                client: None,
            }),
        );
        let poller = Poller::new().map_err(StateErrorKind::PollError)?;
        let mut log_prefix = String::new();
        if let Ok(prefix) = var("WL_PROXY_PREFIX") {
            log_prefix = prefix;
        }
        if self.log_prefix.is_not_empty() {
            if log_prefix.is_not_empty() {
                log_prefix.push_str(" ");
            }
            log_prefix.push_str(&self.log_prefix);
        }
        if log_prefix.is_not_empty() {
            log_prefix = format!("{{{}}} ", log_prefix);
        }
        let state = Rc::new(State {
            poller,
            next_pollable_id: Cell::new(SERVER_ENDPOINT_ID + 1),
            server,
            destroyed: Default::default(),
            handler: Default::default(),
            pollables: RefCell::new(endpoints),
            acceptable_acceptors: Default::default(),
            has_acceptable_acceptors: Default::default(),
            clients_to_kill: Default::default(),
            has_clients_to_kill: Default::default(),
            readable_endpoints: Default::default(),
            has_readable_endpoints: Default::default(),
            flushable_endpoints: Default::default(),
            has_flushable_endpoints: Default::default(),
            interest_update_endpoints: Default::default(),
            has_interest_update_endpoints: Default::default(),
            interest_update_acceptors: Default::default(),
            has_interest_update_acceptors: Default::default(),
            all_proxies: Default::default(),
            next_proxy_id: Default::default(),
            log: self.log,
            log_prefix,
            log_writer: RefCell::new(BufWriter::with_capacity(1024, Fd::new(c::STDERR_FILENO))),
            global_lock_held: Default::default(),
            proxy_stash: Default::default(),
        });
        state.change_interest(&state.server, |i| i | poll::READABLE);
        state
            .poller
            .register(0, &state.server.socket)
            .map_err(StateErrorKind::PollError)?;
        let display = WlDisplay::new(&state, 1);
        display
            .core()
            .set_server_id_unchecked(1, display.clone())
            .unwrap();
        Ok(state)
    }

    pub fn with_server_fd(mut self, fd: &Rc<OwnedFd>) -> Self {
        self.server_fd = Some(fd.clone());
        self
    }

    pub fn with_logging(mut self, log: bool) -> Self {
        self.log = log;
        self
    }

    pub fn with_log_prefix(mut self, prefix: &str) -> Self {
        self.log_prefix = prefix.to_string();
        self
    }
}

pub struct Destructor {
    state: Rc<State>,
    forget: bool,
}

impl Destructor {
    pub fn forget(mut self) {
        self.forget = true;
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        if self.forget {
            return;
        }
        self.state.destroy();
    }
}

pub struct RemoteDestructor {
    destroy: Arc<AtomicBool>,
    _fd: OwnedFd,
    forget: bool,
}

impl RemoteDestructor {
    pub fn forget(mut self) {
        self.forget = true;
    }
}

impl Drop for RemoteDestructor {
    fn drop(&mut self) {
        if self.forget {
            return;
        }
        self.destroy.store(true, Release);
    }
}

pub(crate) struct HandlerLock<'a> {
    state: &'a State,
}

impl Drop for HandlerLock<'_> {
    fn drop(&mut self) {
        self.state.global_lock_held.set(false);
    }
}

impl State {
    fn acquire_handler_lock(&self) -> Result<HandlerLock<'_>, StateErrorKind> {
        if self.global_lock_held.replace(true) {
            return Err(StateErrorKind::RecursiveCall);
        }
        Ok(HandlerLock { state: self })
    }

    pub fn dispatch_blocking(self: &Rc<Self>) -> Result<(), StateError> {
        self.dispatch(None)
    }

    pub fn dispatch_available(self: &Rc<Self>) -> Result<(), StateError> {
        self.dispatch(Some(Duration::from_secs(0)))
    }

    pub fn dispatch(self: &Rc<Self>, timeout: Option<Duration>) -> Result<(), StateError> {
        let lock = self.acquire_handler_lock()?;
        let timeout = timeout
            .and_then(|t| t.as_millis().try_into().ok())
            .unwrap_or(-1);
        let destroy_on_error = on_drop(|| self.destroy());
        if timeout != 0 {
            self.flush_locked(&lock)?;
        }
        self.wait_for_work(&lock, timeout)?;
        self.accept_connections(&lock)?;
        self.read_messages(&lock)?;
        self.flush_locked(&lock)?;
        destroy_on_error.forget();
        Ok(())
    }

    pub fn flush(&self) -> Result<(), StateError> {
        let lock = self.acquire_handler_lock()?;
        let destroy_on_error = on_drop(|| self.destroy());
        self.flush_locked(&lock)?;
        destroy_on_error.forget();
        Ok(())
    }

    fn flush_locked(&self, lock: &HandlerLock<'_>) -> Result<(), StateError> {
        self.perform_writes(lock)?;
        self.kill_clients();
        self.update_interests()?;
        Ok(())
    }

    pub(crate) fn handle_delete_id(&self, id: u32) {
        let object = self.server.objects.borrow_mut().remove(&id).unwrap();
        let core = object.core();
        core.server_obj_id.take();
        self.server.idl.release(id);
        if let Err((e, object)) = object.delete_id() {
            log::warn!(
                "Could not handle a wl_display.delete_id message: {}",
                Report::new(e)
            );
            let _ = object.delete_id();
        }
    }

    pub(crate) fn perform_writes(&self, _: &HandlerLock<'_>) -> Result<(), StateError> {
        if !self.has_flushable_endpoints.get() {
            return Ok(());
        }
        while let Some(ewc) = self.flushable_endpoints.pop() {
            let res = match ewc.endpoint.flush() {
                Ok(r) => r,
                Err(e) => {
                    let is_closed = matches!(e, EndpointError::Flush(TransError::Closed));
                    if let Some(client) = &ewc.client {
                        if !is_closed {
                            log::warn!(
                                "Could not write to client#{}: {}",
                                client.endpoint.id,
                                Report::new(e),
                            );
                        }
                        self.add_client_to_kill(client);
                    } else {
                        if is_closed {
                            return Err(StateErrorKind::ServerHangup.into());
                        }
                        return Err(StateErrorKind::WriteToServer(e).into());
                    }
                    continue;
                }
            };
            match res {
                FlushResult::Done => {
                    ewc.endpoint.flush_queued.set(false);
                    self.change_interest(&ewc.endpoint, |i| i & !poll::WRITABLE);
                }
                FlushResult::Blocked => {
                    self.change_interest(&ewc.endpoint, |i| i | poll::WRITABLE);
                }
            }
        }
        self.has_flushable_endpoints.set(false);
        Ok(())
    }

    pub(crate) fn accept_connections(
        self: &Rc<Self>,
        lock: &HandlerLock<'_>,
    ) -> Result<(), StateError> {
        if !self.has_acceptable_acceptors.get() {
            return Ok(());
        }
        self.check_destroyed()?;
        while let Some(acceptor) = self.acceptable_acceptors.pop() {
            self.interest_update_acceptors.push(acceptor.clone());
            self.has_interest_update_acceptors.set(true);
            const MAX_ACCEPT_PER_ITERATION: usize = 10;
            for _ in 0..MAX_ACCEPT_PER_ITERATION {
                let socket = acceptor
                    .accept()
                    .map_err(StateErrorKind::AcceptConnection)?;
                let Some(socket) = socket else {
                    break;
                };
                self.create_client(Some(lock), &Rc::new(socket))?;
            }
        }
        self.has_acceptable_acceptors.set(false);
        Ok(())
    }

    pub(crate) fn read_messages(&self, lock: &HandlerLock<'_>) -> Result<(), StateError> {
        if !self.has_readable_endpoints.get() {
            return Ok(());
        }
        while let Some(ewc) = self.readable_endpoints.pop() {
            let res = ewc.endpoint.read_messages(lock, ewc.client.as_ref());
            if let Err(e) = res {
                if let Some(client) = &ewc.client {
                    log::error!("Could not handle client message: {}", Report::new(e));
                    self.add_client_to_kill(client);
                } else {
                    return Err(StateErrorKind::DispatchEvents(e).into());
                }
            }
            self.change_interest(&ewc.endpoint, |i| i | poll::READABLE);
        }
        self.has_readable_endpoints.set(false);
        Ok(())
    }

    pub(crate) fn change_interest(&self, endpoint: &Rc<Endpoint>, f: impl FnOnce(u32) -> u32) {
        if self.destroyed.get() {
            return;
        }
        let old = endpoint.desired_interest.get();
        let new = f(old);
        endpoint.desired_interest.set(new);
        if old != new
            && endpoint.current_interest.get() != new
            && !endpoint.interest_update_queued.replace(true)
        {
            self.interest_update_endpoints.push(endpoint.clone());
            self.has_interest_update_endpoints.set(true);
        }
    }

    pub(crate) fn add_flushable_endpoint(
        &self,
        endpoint: &Rc<Endpoint>,
        client: Option<&Rc<Client>>,
    ) {
        if self.destroyed.get() {
            return;
        }
        self.flushable_endpoints.push(EndpointWithClient {
            endpoint: endpoint.clone(),
            client: client.cloned(),
        });
        self.has_flushable_endpoints.set(true);
    }

    pub(crate) fn wait_for_work(
        &self,
        _: &HandlerLock<'_>,
        mut timeout: c::c_int,
    ) -> Result<(), StateError> {
        self.check_destroyed()?;
        let mut events = [PollEvent::default(); poll::MAX_EVENTS];
        let pollables = &mut *self.pollables.borrow_mut();
        loop {
            let n = self
                .poller
                .read_events(timeout, &mut events)
                .map_err(StateErrorKind::PollError)?;
            if n == 0 {
                return Ok(());
            }
            timeout = 0;
            for event in &events[0..n] {
                let id = event.u64;
                let Some(pollable) = pollables.get(&id) else {
                    continue;
                };
                match pollable {
                    Pollable::Endpoint(ewc) => {
                        let events = event.events;
                        if events & poll::ERROR != 0 {
                            if let Some(client) = &ewc.client {
                                self.add_client_to_kill(client);
                            } else {
                                return Err(StateErrorKind::ServerHangup.into());
                            }
                            continue;
                        }
                        ewc.endpoint.current_interest.set(0);
                        self.change_interest(&ewc.endpoint, |i| i & !events);
                        if events & poll::READABLE != 0 {
                            self.readable_endpoints.push(ewc.clone());
                            self.has_readable_endpoints.set(true);
                        }
                        if events & poll::WRITABLE != 0 {
                            self.flushable_endpoints.push(ewc.clone());
                            self.has_flushable_endpoints.set(true);
                        }
                    }
                    Pollable::Acceptor(a) => {
                        self.acceptable_acceptors.push(a.clone());
                        self.has_acceptable_acceptors.set(true);
                    }
                    Pollable::Destructor(_, destroy) => {
                        let destroy = destroy.load(Acquire);
                        pollables.remove(&id);
                        if destroy {
                            return Err(StateErrorKind::RemoteDestroyed.into());
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn add_client_to_kill(&self, client: &Rc<Client>) {
        self.clients_to_kill.push(client.clone());
        self.has_clients_to_kill.set(true);
    }

    pub(crate) fn kill_clients(&self) {
        if !self.has_clients_to_kill.get() {
            return;
        }
        while let Some(client) = self.clients_to_kill.pop() {
            if let Some(handler) = client.handler.borrow_mut().take() {
                handler.disconnected();
            }
            client.kill();
        }
        self.has_clients_to_kill.set(false);
    }

    pub(crate) fn create_pollable_id(&self) -> u64 {
        let id = self.next_pollable_id.get();
        self.next_pollable_id.set(id + 1);
        id
    }

    pub(crate) fn update_interests(&self) -> Result<(), StateError> {
        if self.has_interest_update_endpoints.get() {
            while let Some(endpoint) = self.interest_update_endpoints.pop() {
                endpoint.interest_update_queued.set(false);
                let desired = endpoint.desired_interest.get();
                if desired == endpoint.current_interest.get() {
                    continue;
                }
                self.poller
                    .update_interests(&endpoint.socket, endpoint.id, desired)
                    .map_err(StateErrorKind::PollError)?;
                endpoint.current_interest.set(desired);
            }
            self.has_interest_update_endpoints.set(false);
        }
        if self.has_interest_update_acceptors.get() {
            while let Some(acceptor) = self.interest_update_acceptors.pop() {
                self.poller
                    .update_interests(&acceptor.socket, acceptor.id, poll::READABLE)
                    .map_err(StateErrorKind::PollError)?;
            }
            self.has_interest_update_acceptors.set(false);
        }
        Ok(())
    }

    fn check_destroyed(&self) -> Result<(), StateError> {
        if self.destroyed.get() {
            return Err(StateErrorKind::Destroyed.into());
        }
        Ok(())
    }

    pub fn create_object<P>(self: &Rc<Self>, version: u32) -> Rc<P>
    where
        P: Object + Sized,
    {
        P::new(self, version)
    }

    #[cold]
    pub(crate) fn log(&self, args: fmt::Arguments<'_>) {
        let writer = &mut *self.log_writer.borrow_mut();
        let _ = writer.write_fmt(args);
        let _ = writer.flush();
    }
}

/// These functions can be used to manage sockets associated with this state.
impl State {
    /// Creates a new connection to this proxy.
    ///
    /// The returned file descriptor is the client end of the connection and can be used
    /// with a function such as `wl_display_connect_to_fd` or with the `WAYLAND_SOCKET`
    /// environment variable.
    ///
    /// The [`StateHandler::new_client`] callback will not be invoked.
    pub fn connect(self: &Rc<Self>) -> Result<(Rc<Client>, OwnedFd), StateError> {
        let (server_fd, client_fd) = uapi::socketpair(
            c::AF_UNIX,
            c::SOCK_STREAM | c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
            0,
        )
        .map_err(|e| StateErrorKind::Socketpair(e.into()))?;
        let client = self.create_client(None, &Rc::new(server_fd.into()))?;
        Ok((client, client_fd.into()))
    }

    /// Creates a new connection to this proxy from an existing socket.
    ///
    /// The file descriptor should be the server end of the connection. It can be created
    /// with a function such as `socketpair` or by accepting a connection from a
    /// file-system socket.
    ///
    /// The [`StateHandler::new_client`] callback will not be invoked.
    pub fn add_client(self: &Rc<Self>, socket: &Rc<OwnedFd>) -> Result<Rc<Client>, StateError> {
        self.create_client(None, socket)
    }

    /// Creates a new file-system acceptor and starts listening for connections.
    ///
    /// See [`Acceptor::new`] for the meaning of the `max_tries` parameter.
    ///
    /// Calling [`State::dispatch`] will automatically accept connections from this
    /// acceptor. The [`StateHandler::new_client`] callback will be invoked when this
    /// happens.
    pub fn create_acceptor(&self, max_tries: u32) -> Result<Rc<Acceptor>, StateError> {
        self.check_destroyed()?;
        let id = self.create_pollable_id();
        let acceptor =
            Acceptor::create(id, max_tries, true).map_err(StateErrorKind::CreateAcceptor)?;
        self.poller
            .register(id, &acceptor.socket)
            .map_err(StateErrorKind::PollError)?;
        self.update_interests()?;
        self.interest_update_acceptors.push(acceptor.clone());
        self.has_interest_update_acceptors.set(true);
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Acceptor(acceptor.clone()));
        Ok(acceptor)
    }

    fn create_client(
        self: &Rc<Self>,
        lock: Option<&HandlerLock<'_>>,
        socket: &Rc<OwnedFd>,
    ) -> Result<Rc<Client>, StateError> {
        self.check_destroyed()?;
        let id = self.create_pollable_id();
        self.poller
            .register(id, &socket)
            .map_err(StateErrorKind::PollError)?;
        let endpoint = Endpoint::new(id, socket);
        self.change_interest(&endpoint, |i| i | poll::READABLE);
        self.update_interests()?;
        let display = WlDisplay::new(self, 1);
        display.core().server_obj_id.set(Some(1));
        let client = Rc::new(Client {
            state: self.clone(),
            endpoint: endpoint.clone(),
            display,
            destroyed: Cell::new(false),
            handler: Default::default(),
        });
        client
            .display
            .core()
            .set_client_id(&client, 1, client.display.clone())
            .unwrap();
        self.pollables.borrow_mut().insert(
            id,
            Pollable::Endpoint(EndpointWithClient {
                endpoint,
                client: Some(client.clone()),
            }),
        );
        if lock.is_some()
            && let Some(handler) = &mut *self.handler.borrow_mut()
        {
            handler.new_client(&client);
        }
        Ok(client)
    }
}

/// These functions can be used to manipulate the [`StateHandler`] of this state.
///
/// These functions can be called at any time, even from within a handler callback. In
/// that case, the handler is replaced as soon as the callback returns.
impl State {
    /// Unsets the handler.
    pub fn unset_handler(&self) {
        self.handler.set(None);
    }

    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl StateHandler) {
        self.set_boxed_handler(Box::new(handler))
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn StateHandler>) {
        if self.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

/// These functions can be used to check the state status and to destroy the state.
impl State {
    /// Returns whether this state is not destroyed.
    ///
    /// This is the same as `!self.is_destroyed()`.
    pub fn is_not_destroyed(&self) -> bool {
        !self.is_destroyed()
    }

    /// Returns whether the state is destroyed.
    ///
    /// If the state is destroyed, most functions that can return an error will return an
    /// error saying that the state is already destroyed.
    ///
    /// This function or [`Self::is_not_destroyed`] should be used before dispatching the
    /// state.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use error_reporter::Report;
    /// # use wl_proxy::state::State;
    /// #
    /// # fn f(state: &Rc<State>) {
    /// while state.is_not_destroyed() {
    ///     if let Err(e) = state.dispatch_blocking() {
    ///         log::error!("Could not dispatch the state: {}", Report::new(e));
    ///     }
    /// }
    /// # }
    /// ```
    pub fn is_destroyed(&self) -> bool {
        self.destroyed.get()
    }

    /// Destroys this state.
    ///
    /// This function unsets all handlers and destroys all clients. You should drop the
    /// state after calling this function.
    pub fn destroy(&self) {
        if self.destroyed.replace(true) {
            return;
        }
        let proxies = &mut *self.proxy_stash.borrow();
        for pollable in self.pollables.borrow().values() {
            if let Pollable::Endpoint(ewc) = pollable {
                if let Some(c) = &ewc.client {
                    c.destroyed.set(true);
                }
                proxies.extend(ewc.endpoint.objects.borrow_mut().drain().map(|v| v.1));
            }
        }
        proxies.clear();
        for proxy in self.all_proxies.borrow().values() {
            if let Some(proxy) = proxy.upgrade() {
                proxies.push(proxy);
            }
        }
        for proxy in proxies {
            proxy.unset_handler();
            proxy.core().client.take();
        }
        self.handler.set(None);
        self.pollables.borrow_mut().clear();
        self.acceptable_acceptors.take();
        self.clients_to_kill.take();
        self.readable_endpoints.take();
        self.flushable_endpoints.take();
        self.interest_update_endpoints.take();
        self.interest_update_acceptors.take();
        self.all_proxies.borrow_mut().clear();
        // Ensure that the poll fd stays permanently readable.
        let _ = self.create_remote_destructor();
    }

    /// Creates a RAII destructor for this state.
    ///
    /// Dropping the destructor will automatically call [`State::destroy`] unless you
    /// first call [`Destructor::forget`].
    ///
    /// State objects contain reference cycles that must be cleared manually to release
    /// the associated resources. Dropping the [`State`] is usually not sufficient to do
    /// this. Instead, [`State::destroy`] must be called manually. This function can be
    /// used to accomplish this in an application that otherwise relies on RAII semantics.
    ///
    /// Ensure that the destructor is itself not part of a reference cycle.
    pub fn create_destructor(self: &Rc<Self>) -> Destructor {
        Destructor {
            state: self.clone(),
            forget: false,
        }
    }

    /// Creates a `Sync+Send` RAII destructor for this state.
    ///
    /// This function is similar to [`State::create_destructor`] but the returned
    /// destructor implements `Sync+Send`. This destructor can therefore be used to
    /// destroy states running in a different thread.
    pub fn create_remote_destructor(&self) -> Result<RemoteDestructor, StateError> {
        let (r, w) = pipe().map_err(StateErrorKind::CreatePipe)?;
        let r: OwnedFd = r.into();
        let id = self.create_pollable_id();
        self.poller
            .register(id, &r)
            .map_err(StateErrorKind::PollError)?;
        let destroy = Arc::new(AtomicBool::new(false));
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Destructor(r, destroy.clone()));
        Ok(RemoteDestructor {
            destroy,
            _fd: w.into(),
            forget: false,
        })
    }
}

/// A handler for events created by a state.
pub trait StateHandler: 'static {
    fn new_client(&mut self, client: &Rc<Client>);
}
