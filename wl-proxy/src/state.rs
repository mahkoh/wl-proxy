use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        client::Client,
        endpoint::{Endpoint, EndpointError},
        protocols::wayland::wl_display::WlDisplay,
        proxy::{Proxy, ProxyPrivate},
        trans::{FlushResult, TransError},
        utils::{handler_holder::HandlerHolder, stack::Stack, stash::Stash, xrd::xrd},
    },
    error_reporter::Report,
    isnt::std_1::primitive::IsntStrExt,
    run_on_drop::on_drop,
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        env::var,
        error::Error,
        fmt,
        io::{self, BufWriter, Write, pipe},
        os::fd::{AsRawFd, OwnedFd},
        rc::{Rc, Weak},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering::Relaxed},
        },
        time::Duration,
    },
    thiserror::Error,
    uapi::{
        Errno, Fd,
        c::{self, EPOLL_CTL_ADD, sockaddr_un},
    },
};

#[derive(Debug, Error)]
pub enum StateError {
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
    #[error("could not create epoll fd")]
    CreateEpoll(#[source] io::Error),
    #[error("could not read epoll events")]
    ReadEpoll(#[source] io::Error),
    #[error("could not register socket with epoll")]
    AddEpoll(#[source] io::Error),
    #[error("could not update epoll interests")]
    UpdateEpoll(#[source] io::Error),
    #[error("could not create a socket pair")]
    Socketpair(#[source] io::Error),
    #[error(transparent)]
    CreateAcceptor(AcceptorError),
    #[error("could not accept a new connection")]
    AcceptConnection(AcceptorError),
    #[error("could not create a pipe")]
    CreatePipe(#[source] io::Error),
    #[error("could not read WAYLAND_DISPLAY environment variable")]
    WaylandDisplay,
    #[error("XDG_RUNTIME_DIR is not set")]
    XrdNotSet,
    #[error("the socket path is too long")]
    SocketPathTooLong,
    #[error("could not create a socket")]
    CreateSocket(#[source] io::Error),
    #[error("could not connect to {0}")]
    Connect(String, #[source] io::Error),
    #[error("could not spawn a thread")]
    SpawnThread(#[source] io::Error),
    #[error("spawned thread panicked")]
    SpawnThreadPanic,
    #[error("could not initialize the spawned state")]
    InitThreadState(#[source] Box<dyn Error + Send + Sync>),
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
    epoll: uapi::OwnedFd,
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
    pub(crate) all_proxies: RefCell<HashMap<u64, Weak<dyn Proxy>>>,
    pub(crate) next_proxy_id: Cell<u64>,
    pub(crate) log: bool,
    pub(crate) log_prefix: String,
    log_writer: RefCell<BufWriter<Fd>>,
    global_lock_held: Cell<bool>,
    pub(crate) proxy_stash: Stash<Rc<dyn Proxy>>,
}

pub struct StateBuilder {
    server_fd: Option<OwnedFd>,
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
            _ => {
                let mut name = var("WAYLAND_DISPLAY")
                    .ok()
                    .ok_or(StateError::WaylandDisplay)?;
                if name.is_empty() {
                    return Err(StateError::WaylandDisplay);
                }
                if !name.starts_with("/") {
                    let Some(xrd) = xrd() else {
                        return Err(StateError::XrdNotSet);
                    };
                    name = format!("{xrd}/{name}");
                }
                let mut addr = sockaddr_un {
                    sun_family: c::AF_UNIX as _,
                    sun_path: [0; 108],
                };
                if name.len() > addr.sun_path.len() - 1 {
                    return Err(StateError::SocketPathTooLong);
                }
                let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
                sun_path[..name.len()].copy_from_slice(name.as_bytes());
                sun_path[name.len()] = 0;
                let socket = uapi::socket(
                    c::AF_UNIX,
                    c::SOCK_STREAM | c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
                    0,
                )
                .map_err(|e| StateError::CreateSocket(e.into()))?;
                uapi::connect(socket.raw(), &addr)
                    .map_err(|e| StateError::Connect(name.to_string(), e.into()))?;
                socket.into()
            }
        };
        const SERVER_ENDPOINT_ID: u64 = 0;
        let server = Endpoint::new(SERVER_ENDPOINT_ID, server_fd);
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
        let epoll =
            uapi::epoll_create1(c::EPOLL_CLOEXEC).map_err(|e| StateError::CreateEpoll(e.into()))?;
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
            epoll,
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
        state.change_interest(&state.server, |i| i | c::EPOLLIN);
        state.register_socket(0, &state.server.socket)?;
        let display = WlDisplay::new(&state, 1);
        display
            .core()
            .set_server_id_unchecked(1, display.clone())
            .unwrap();
        Ok(state)
    }

    pub fn with_server_fd(mut self, fd: OwnedFd) -> Self {
        self.server_fd = Some(fd);
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
        self.destroy.store(true, Relaxed);
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
    pub fn new() -> Result<Rc<Self>, StateError> {
        StateBuilder::default().build()
    }

    fn acquire_handler_lock(&self) -> Result<HandlerLock<'_>, StateError> {
        if self.global_lock_held.replace(true) {
            return Err(StateError::RecursiveCall);
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
        let proxy = self.server.objects.borrow_mut().remove(&id).unwrap();
        let core = proxy.core();
        core.zombie.set(false);
        core.server_obj_id.take();
        self.server.idl.release(id);
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
                                "could not write to client#{}: {}",
                                client.endpoint.id,
                                Report::new(e),
                            );
                        }
                        self.add_client_to_kill(client);
                    } else {
                        if is_closed {
                            return Err(StateError::ServerHangup);
                        }
                        return Err(StateError::WriteToServer(e));
                    }
                    continue;
                }
            };
            match res {
                FlushResult::Done => {
                    ewc.endpoint.flush_queued.set(false);
                    self.change_interest(&ewc.endpoint, |i| i & !c::EPOLLOUT);
                }
                FlushResult::Blocked => {
                    self.change_interest(&ewc.endpoint, |i| i | c::EPOLLOUT);
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
                let socket = acceptor.accept().map_err(StateError::AcceptConnection)?;
                let Some(socket) = socket else {
                    break;
                };
                self.create_client(Some(lock), socket)?;
            }
        }
        self.has_acceptable_acceptors.set(false);
        Ok(())
    }

    pub(crate) fn create_client(
        self: &Rc<Self>,
        lock: Option<&HandlerLock<'_>>,
        socket: OwnedFd,
    ) -> Result<Rc<Client>, StateError> {
        self.check_destroyed()?;
        let id = self.create_pollable_id();
        self.register_socket(id, &socket)?;
        let endpoint = Endpoint::new(id, socket);
        self.change_interest(&endpoint, |i| i | c::EPOLLIN);
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
            && let Some(handler) = &mut *self.handler.borrow()
        {
            handler.new_client(&client);
        }
        Ok(client)
    }

    pub(crate) fn read_messages(&self, lock: &HandlerLock<'_>) -> Result<(), StateError> {
        if !self.has_readable_endpoints.get() {
            return Ok(());
        }
        while let Some(ewc) = self.readable_endpoints.pop() {
            let res = ewc.endpoint.read_messages(lock, ewc.client.as_ref());
            if let Err(e) = res {
                if let Some(client) = &ewc.client {
                    self.add_client_to_kill(client);
                } else {
                    return Err(StateError::DispatchEvents(e));
                }
            }
            self.change_interest(&ewc.endpoint, |i| i | c::EPOLLIN);
        }
        self.has_readable_endpoints.set(false);
        Ok(())
    }

    pub(crate) fn change_interest(
        &self,
        endpoint: &Rc<Endpoint>,
        f: impl FnOnce(c::c_int) -> c::c_int,
    ) {
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

    pub fn create_destructor(self: &Rc<Self>) -> Destructor {
        Destructor {
            state: self.clone(),
            forget: false,
        }
    }

    pub fn create_remote_destructor(&self) -> Result<RemoteDestructor, StateError> {
        let (r, w) = pipe().map_err(StateError::CreatePipe)?;
        let id = self.create_pollable_id();
        let event = c::epoll_event { events: 0, u64: id };
        uapi::epoll_ctl(
            self.epoll.as_raw_fd(),
            EPOLL_CTL_ADD,
            r.as_raw_fd(),
            Some(&event),
        )
        .map_err(|e| StateError::UpdateEpoll(e.into()))?;
        let destroy = Arc::new(AtomicBool::new(false));
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Destructor(r.into(), destroy.clone()));
        Ok(RemoteDestructor {
            destroy,
            _fd: w.into(),
            forget: false,
        })
    }

    pub(crate) fn wait_for_work(
        &self,
        _: &HandlerLock<'_>,
        mut timeout: c::c_int,
    ) -> Result<(), StateError> {
        self.check_destroyed()?;
        let mut events = [c::epoll_event { events: 0, u64: 0 }; 16];
        let pollables = &mut *self.pollables.borrow_mut();
        loop {
            let res = uapi::epoll_wait(self.epoll.as_raw_fd(), &mut events, timeout);
            let n = match res {
                Ok(0) => return Ok(()),
                Ok(n) => n,
                Err(Errno(c::EINTR)) => continue,
                Err(e) => return Err(StateError::ReadEpoll(e.into())),
            };
            timeout = 0;
            for event in &events[0..n] {
                let id = event.u64;
                let Some(pollable) = pollables.get(&id) else {
                    continue;
                };
                match pollable {
                    Pollable::Endpoint(ewc) => {
                        let events = event.events as c::c_int;
                        if events & (c::EPOLLHUP | c::EPOLLERR) != 0 {
                            if let Some(client) = &ewc.client {
                                self.add_client_to_kill(client);
                            } else {
                                return Err(StateError::ServerHangup);
                            }
                            continue;
                        }
                        ewc.endpoint.current_interest.set(0);
                        self.change_interest(&ewc.endpoint, |i| i & !events);
                        if events & c::EPOLLIN != 0 {
                            self.readable_endpoints.push(ewc.clone());
                            self.has_readable_endpoints.set(true);
                        }
                        if events & c::EPOLLOUT != 0 {
                            self.flushable_endpoints.push(ewc.clone());
                            self.has_flushable_endpoints.set(true);
                        }
                    }
                    Pollable::Acceptor(a) => {
                        self.acceptable_acceptors.push(a.clone());
                        self.has_acceptable_acceptors.set(true);
                    }
                    Pollable::Destructor(_, destroy) => {
                        let destroy = destroy.load(Relaxed);
                        pollables.remove(&id);
                        if destroy {
                            return Err(StateError::RemoteDestroyed);
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
            if let Some(handler) = client.handler.borrow().take() {
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
        let epoll_ctl = |socket: &OwnedFd, event: &c::epoll_event| {
            uapi::epoll_ctl(
                self.epoll.as_raw_fd(),
                c::EPOLL_CTL_MOD,
                socket.as_raw_fd(),
                Some(event),
            )
            .map_err(|e| StateError::UpdateEpoll(io::Error::from_raw_os_error(e.0)))
        };
        if self.has_interest_update_endpoints.get() {
            while let Some(endpoint) = self.interest_update_endpoints.pop() {
                endpoint.interest_update_queued.set(false);
                let desired = endpoint.desired_interest.get();
                if desired == endpoint.current_interest.get() {
                    continue;
                }
                let event = c::epoll_event {
                    events: (desired | c::EPOLLONESHOT) as _,
                    u64: endpoint.id,
                };
                epoll_ctl(&endpoint.socket, &event)?;
                endpoint.current_interest.set(desired);
            }
            self.has_interest_update_endpoints.set(false);
        }
        if self.has_interest_update_acceptors.get() {
            while let Some(acceptor) = self.interest_update_acceptors.pop() {
                let event = c::epoll_event {
                    events: (c::EPOLLIN | c::EPOLLONESHOT) as _,
                    u64: acceptor.id,
                };
                epoll_ctl(&acceptor.socket, &event)?;
            }
            self.has_interest_update_acceptors.set(false);
        }
        Ok(())
    }

    fn register_socket(&self, id: u64, socket: &OwnedFd) -> Result<(), StateError> {
        let event = c::epoll_event { events: 0, u64: id };
        uapi::epoll_ctl(
            self.epoll.as_raw_fd(),
            c::EPOLL_CTL_ADD,
            socket.as_raw_fd(),
            Some(&event),
        )
        .map_err(|e| StateError::AddEpoll(e.into()))
    }

    pub fn connect(self: &Rc<Self>) -> Result<(Rc<Client>, OwnedFd), StateError> {
        let (server_fd, client_fd) = uapi::socketpair(
            c::AF_UNIX,
            c::SOCK_STREAM | c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
            0,
        )
        .map_err(|e| StateError::Socketpair(e.into()))?;
        let client = self.create_client(None, server_fd.into())?;
        Ok((client, client_fd.into()))
    }

    pub fn add_client(self: &Rc<Self>, socket: OwnedFd) -> Result<Rc<Client>, StateError> {
        self.create_client(None, socket)
    }

    pub fn create_acceptor(&self, max_tries: u32) -> Result<Rc<Acceptor>, StateError> {
        self.check_destroyed()?;
        let id = self.create_pollable_id();
        let acceptor = Acceptor::create(id, max_tries, true).map_err(StateError::CreateAcceptor)?;
        self.register_socket(id, &acceptor.socket)?;
        self.interest_update_acceptors.push(acceptor.clone());
        self.has_interest_update_acceptors.set(true);
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Acceptor(acceptor.clone()));
        Ok(acceptor)
    }

    fn check_destroyed(&self) -> Result<(), StateError> {
        if self.destroyed.get() {
            return Err(StateError::Destroyed);
        }
        Ok(())
    }

    pub fn set_handler(&self, handler: impl StateHandler) {
        self.set_boxed_handler(Box::new(handler))
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn StateHandler>) {
        if self.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }

    pub fn create_proxy<P>(self: &Rc<Self>, version: u32) -> Rc<P>
    where
        P: Proxy + Sized,
    {
        P::new(self, version)
    }

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
    }

    #[cold]
    pub(crate) fn log(&self, args: fmt::Arguments<'_>) {
        let writer = &mut *self.log_writer.borrow_mut();
        let _ = writer.write_fmt(args);
        let _ = writer.flush();
    }
}

pub trait StateHandler: 'static {
    fn new_client(&mut self, client: &Rc<Client>);
}
