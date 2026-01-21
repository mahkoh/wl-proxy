//! The proxy state.

use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        baseline::Baseline,
        client::Client,
        endpoint::{Endpoint, EndpointError},
        handler::HandlerHolder,
        object::{Object, ObjectCoreApi, ObjectErrorKind, ObjectPrivate},
        poll::{self, PollError, PollEvent, Poller},
        protocols::wayland::wl_display::WlDisplay,
        trans::{FlushResult, TransError},
        utils::{
            env::{WAYLAND_DISPLAY, WAYLAND_SOCKET, XDG_RUNTIME_DIR},
            stack::Stack,
            stash::Stash,
        },
    },
    error_reporter::Report,
    run_on_drop::on_drop,
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        io::{self, pipe},
        os::fd::{AsFd, OwnedFd},
        rc::{Rc, Weak},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering::Acquire},
        },
        time::Duration,
    },
    thiserror::Error,
    uapi::c,
};
pub use {
    builder::StateBuilder,
    destructor::{Destructor, RemoteDestructor},
};

mod builder;
mod destructor;
#[cfg(test)]
mod tests;

/// An error emitted by a [`State`].
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
    #[error("the display name is empty")]
    WaylandDisplayEmpty,
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

/// The proxy state.
///
/// This type represents a connection to a server and any number of clients connected to
/// this proxy.
///
/// This type can be constructed by using a [`StateBuilder`].
///
/// # Example
///
/// ```
/// # use std::rc::Rc;
/// # use wl_proxy::baseline::Baseline;
/// # use wl_proxy::client::{Client, ClientHandler};
/// # use wl_proxy::protocols::wayland::wl_display::{WlDisplay, WlDisplayHandler};
/// # use wl_proxy::protocols::wayland::wl_registry::WlRegistry;
/// # use wl_proxy::state::{State, StateBuilder, StateHandler};
/// # fn f() {
/// let state = State::builder(Baseline::ALL_OF_THEM).build().unwrap();
/// let acceptor = state.create_acceptor(1000).unwrap();
/// eprintln!("{}", acceptor.display());
/// loop {
///     state.dispatch_blocking().unwrap();
/// }
///
/// struct StateHandlerImpl;
///
/// impl StateHandler for StateHandlerImpl {
///     fn new_client(&mut self, client: &Rc<Client>) {
///         eprintln!("Client connected");
///         client.set_handler(ClientHandlerImpl);
///         client.display().set_handler(DisplayHandler);
///     }
/// }
///
/// struct ClientHandlerImpl;
///
/// impl ClientHandler for ClientHandlerImpl {
///     fn disconnected(self: Box<Self>) {
///         eprintln!("Client disconnected");
///     }
/// }
///
/// struct DisplayHandler;
///
/// impl WlDisplayHandler for DisplayHandler {
///     fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
///         eprintln!("get_registry called");
///         let _ = slf.send_get_registry(registry);
///     }
/// }
/// # }
/// ```
pub struct State {
    pub(crate) baseline: Baseline,
    poller: Poller,
    next_pollable_id: Cell<u64>,
    pub(crate) server: Option<Rc<Endpoint>>,
    pub(crate) destroyed: Cell<bool>,
    handler: HandlerHolder<dyn StateHandler>,
    pollables: RefCell<HashMap<u64, Pollable>>,
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
    pub(crate) all_objects: RefCell<HashMap<u64, Weak<dyn Object>>>,
    pub(crate) next_object_id: Cell<u64>,
    #[cfg(feature = "logging")]
    pub(crate) log: bool,
    #[cfg(feature = "logging")]
    pub(crate) log_prefix: String,
    #[cfg(feature = "logging")]
    log_writer: RefCell<io::BufWriter<uapi::Fd>>,
    global_lock_held: Cell<bool>,
    pub(crate) object_stash: Stash<Rc<dyn Object>>,
    pub(crate) forward_to_client: Cell<bool>,
    pub(crate) forward_to_server: Cell<bool>,
}

/// A handler for events emitted by a [`State`].
pub trait StateHandler: 'static {
    /// A new client has connected.
    ///
    /// This event is not emitted if the connection is created explicitly via
    /// [`State::connect`] or [`State::add_client`].
    fn new_client(&mut self, client: &Rc<Client>) {
        let _ = client;
    }

    /// The server has sent a wl_display.error event.
    ///
    /// Such errors are fatal.
    ///
    /// The object can be `None` if the error is sent on an object that has already been
    /// deleted.
    fn display_error(
        self: Box<Self>,
        object: Option<&Rc<dyn Object>>,
        server_id: u32,
        error: u32,
        msg: &str,
    ) {
        let _ = object;
        let _ = server_id;
        let _ = error;
        let _ = msg;
    }
}

enum Pollable {
    Endpoint(EndpointWithClient),
    Acceptor(Rc<Acceptor>),
    Destructor(OwnedFd, Arc<AtomicBool>),
}

#[derive(Clone)]
struct EndpointWithClient {
    endpoint: Rc<Endpoint>,
    client: Option<Rc<Client>>,
}

pub(crate) struct HandlerLock<'a> {
    state: &'a State,
}

impl State {
    pub(crate) fn remove_endpoint(&self, endpoint: &Endpoint) {
        self.pollables.borrow_mut().remove(&endpoint.id);
        self.poller.unregister(endpoint.socket.as_fd());
        endpoint.unregistered.set(true);
    }

    fn acquire_handler_lock(&self) -> Result<HandlerLock<'_>, StateErrorKind> {
        if self.global_lock_held.replace(true) {
            return Err(StateErrorKind::RecursiveCall);
        }
        Ok(HandlerLock { state: self })
    }

    fn flush_locked(&self, lock: &HandlerLock<'_>) -> Result<bool, StateError> {
        let mut did_work = false;
        did_work |= self.perform_writes(lock)?;
        did_work |= self.kill_clients();
        self.update_interests()?;
        Ok(did_work)
    }

    pub(crate) fn handle_delete_id(&self, server: &Endpoint, id: u32) {
        let object = server.objects.borrow_mut().remove(&id).unwrap();
        let core = object.core();
        core.server_obj_id.take();
        server.idl.release(id);
        if let Err((e, object)) = object.delete_id() {
            log::warn!(
                "Could not handle a wl_display.delete_id message: {}",
                Report::new(e),
            );
            let _ = object.core().try_delete_id();
        }
    }

    fn perform_writes(&self, _: &HandlerLock<'_>) -> Result<bool, StateError> {
        if !self.has_flushable_endpoints.get() {
            return Ok(false);
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
        Ok(true)
    }

    fn accept_connections(self: &Rc<Self>, lock: &HandlerLock<'_>) -> Result<bool, StateError> {
        if !self.has_acceptable_acceptors.get() {
            return Ok(false);
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
        Ok(true)
    }

    fn read_messages(&self, lock: &HandlerLock<'_>) -> Result<bool, StateError> {
        if !self.has_readable_endpoints.get() {
            return Ok(false);
        }
        while let Some(ewc) = self.readable_endpoints.pop() {
            let res = ewc.endpoint.read_messages(lock, ewc.client.as_ref());
            if let Err(e) = res {
                if let Some(client) = &ewc.client {
                    log::error!("Could not handle client message: {}", Report::new(e));
                    self.add_client_to_kill(client);
                } else {
                    if let EndpointError::HandleMessage(msg) = &e
                        && let ObjectErrorKind::ServerError(object, server_id, error, msg) =
                            &msg.source.0
                        && let Some(handler) = self.handler.borrow_mut().take()
                    {
                        handler.display_error(object.as_ref(), *server_id, *error, &msg.0)
                    }
                    return Err(StateErrorKind::DispatchEvents(e).into());
                }
            }
            self.change_interest(&ewc.endpoint, |i| i | poll::READABLE);
        }
        self.has_readable_endpoints.set(false);
        Ok(true)
    }

    fn change_interest(&self, endpoint: &Rc<Endpoint>, f: impl FnOnce(u32) -> u32) {
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

    fn wait_for_work(&self, _: &HandlerLock<'_>, mut timeout: c::c_int) -> Result<(), StateError> {
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
                    Pollable::Destructor(fd, destroy) => {
                        let destroy = destroy.load(Acquire);
                        self.poller.unregister(fd.as_fd());
                        pollables.remove(&id);
                        if destroy {
                            return Err(StateErrorKind::RemoteDestroyed.into());
                        }
                    }
                }
            }
        }
    }

    fn add_client_to_kill(&self, client: &Rc<Client>) {
        self.clients_to_kill.push(client.clone());
        self.has_clients_to_kill.set(true);
    }

    fn kill_clients(&self) -> bool {
        if !self.has_clients_to_kill.get() {
            return false;
        }
        while let Some(client) = self.clients_to_kill.pop() {
            if let Some(handler) = client.handler.borrow_mut().take() {
                handler.disconnected();
            }
            client.disconnect();
        }
        self.has_clients_to_kill.set(false);
        true
    }

    fn create_pollable_id(&self) -> u64 {
        let id = self.next_pollable_id.get();
        self.next_pollable_id.set(id + 1);
        id
    }

    fn update_interests(&self) -> Result<(), StateError> {
        if self.has_interest_update_endpoints.get() {
            while let Some(endpoint) = self.interest_update_endpoints.pop() {
                endpoint.interest_update_queued.set(false);
                let desired = endpoint.desired_interest.get();
                if desired == endpoint.current_interest.get() {
                    continue;
                }
                if endpoint.unregistered.get() {
                    continue;
                }
                self.poller
                    .update_interests(endpoint.id, endpoint.socket.as_fd(), desired)
                    .map_err(StateErrorKind::PollError)?;
                endpoint.current_interest.set(desired);
            }
            self.has_interest_update_endpoints.set(false);
        }
        if self.has_interest_update_acceptors.get() {
            while let Some(acceptor) = self.interest_update_acceptors.pop() {
                self.poller
                    .update_interests(acceptor.id, acceptor.socket.as_fd(), poll::READABLE)
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

    #[cfg(feature = "logging")]
    #[cold]
    pub(crate) fn log(&self, args: std::fmt::Arguments<'_>) {
        use std::io::Write;
        let writer = &mut *self.log_writer.borrow_mut();
        let _ = writer.write_fmt(args);
        let _ = writer.flush();
    }
}

/// These functions can be used to create a new state.
impl State {
    /// Creates a new [`StateBuilder`].
    pub fn builder(baseline: Baseline) -> StateBuilder {
        StateBuilder::new(baseline)
    }
}

/// These functions can be used to dispatch and flush messages.
impl State {
    /// Performs a blocking dispatch.
    ///
    /// This is a shorthand for `self.dispatch(None)`.
    pub fn dispatch_blocking(self: &Rc<Self>) -> Result<bool, StateError> {
        self.dispatch(None)
    }

    /// Performs a non-blocking dispatch.
    ///
    /// This is a shorthand for `self.dispatch(Some(Duration::from_secs(0))`.
    pub fn dispatch_available(self: &Rc<Self>) -> Result<bool, StateError> {
        self.dispatch(Some(Duration::from_secs(0)))
    }

    /// Performs a dispatch.
    ///
    /// The timeout determines how long this function will wait for new events. If the
    /// timeout is `None`, then it will wait indefinitely. If the timeout is `0`, then
    /// it will only process currently available events.
    ///
    /// If the timeout is not `0`, then outgoing messages will be flushed before waiting.
    ///
    /// Outgoing messages will be flushed immediately before this function returns.
    ///
    /// The return value indicates if any work was performed.
    ///
    /// This function is not reentrant. It should not be called from within a callback.
    /// Trying to do so will cause it to return an error immediately and the state will
    /// be otherwise unchanged.
    pub fn dispatch(self: &Rc<Self>, timeout: Option<Duration>) -> Result<bool, StateError> {
        let mut did_work = false;
        let lock = self.acquire_handler_lock()?;
        let timeout = timeout
            .and_then(|t| t.as_millis().try_into().ok())
            .unwrap_or(-1);
        let destroy_on_error = on_drop(|| self.destroy());
        if timeout != 0 {
            did_work |= self.flush_locked(&lock)?;
        }
        self.wait_for_work(&lock, timeout)?;
        did_work |= self.accept_connections(&lock)?;
        did_work |= self.read_messages(&lock)?;
        did_work |= self.flush_locked(&lock)?;
        destroy_on_error.forget();
        Ok(did_work)
    }
}

impl State {
    /// Returns a file descriptor that can be used with epoll or similar.
    ///
    /// If this file descriptor becomes readable, the state should be dispatched.
    /// [`Self::before_poll`] should be used before going to sleep.
    ///
    /// This function always returns the same file descriptor.
    pub fn poll_fd(&self) -> &Rc<OwnedFd> {
        self.poller.fd()
    }

    /// Prepares the state for an external poll operation.
    ///
    /// If [`Self::poll_fd`] is used, this function should be called immediately before
    /// going to sleep. Otherwise, outgoing messages might not be flushed.
    ///
    /// ```
    /// # use std::os::fd::OwnedFd;
    /// # use std::rc::Rc;
    /// # use wl_proxy::state::State;
    /// # fn poll(fd: &OwnedFd) { }
    /// # fn f(state: &Rc<State>) {
    /// loop {
    ///     state.before_poll().unwrap();
    ///     poll(state.poll_fd());
    ///     state.dispatch_available().unwrap();
    /// }
    /// # }
    /// ```
    pub fn before_poll(&self) -> Result<(), StateError> {
        let lock = self.acquire_handler_lock()?;
        let destroy_on_error = on_drop(|| self.destroy());
        self.flush_locked(&lock)?;
        destroy_on_error.forget();
        Ok(())
    }
}

/// These functions can be used to manipulate objects.
impl State {
    /// Creates a new object.
    ///
    /// The new object is not associated with a client ID or a server ID. It can become
    /// associated with a client ID by sending an event with a `new_id` parameter. It can
    /// become associated with a server ID by sending a request with a `new_id` parameter.
    ///
    /// The object can only be associated with one client at a time. The association with
    /// a client is removed when the object is used in a destructor event.
    ///
    /// This function does not enforce that the version is less than or equal to the
    /// maximum version supported by this crate. Using a version that exceeds tha maximum
    /// supported version can cause a protocol error if the client sends a request that is
    /// not available in the maximum supported protocol version or if the server sends an
    /// event that is not available in the maximum supported protocol version.
    pub fn create_object<P>(self: &Rc<Self>, version: u32) -> Rc<P>
    where
        P: Object,
    {
        P::new(self, version)
    }

    /// Returns a wl_display object.
    pub fn display(self: &Rc<Self>) -> Rc<WlDisplay> {
        let display = WlDisplay::new(self, 1);
        if self.server.is_some() {
            display.core().server_obj_id.set(Some(1));
        }
        display
    }

    /// Changes the default forward-to-client setting.
    ///
    /// This affects objects created after this call. See
    /// [`ObjectCoreApi::set_forward_to_client`].
    pub fn set_default_forward_to_client(&self, enabled: bool) {
        self.forward_to_client.set(enabled);
    }

    /// Changes the default forward-to-server setting.
    ///
    /// This affects objects created after this call. See
    /// [`ObjectCoreApi::set_forward_to_server`].
    pub fn set_default_forward_to_server(&self, enabled: bool) {
        self.forward_to_server.set(enabled);
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
            .register(id, acceptor.socket.as_fd())
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
            .register(id, socket.as_fd())
            .map_err(StateErrorKind::PollError)?;
        let endpoint = Endpoint::new(id, socket);
        self.change_interest(&endpoint, |i| i | poll::READABLE);
        self.update_interests()?;
        let client = Rc::new(Client {
            state: self.clone(),
            endpoint: endpoint.clone(),
            display: self.display(),
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
        let objects = &mut *self.object_stash.borrow();
        for pollable in self.pollables.borrow().values() {
            let fd = match pollable {
                Pollable::Endpoint(ewc) => {
                    if let Some(c) = &ewc.client {
                        c.destroyed.set(true);
                    }
                    objects.extend(ewc.endpoint.objects.borrow_mut().drain().map(|v| v.1));
                    &ewc.endpoint.socket
                }
                Pollable::Acceptor(a) => &a.socket,
                Pollable::Destructor(fd, _) => fd,
            };
            self.poller.unregister(fd.as_fd());
        }
        objects.clear();
        for object in self.all_objects.borrow().values() {
            if let Some(object) = object.upgrade() {
                objects.push(object);
            }
        }
        for object in objects {
            object.unset_handler();
            object.core().client.take();
        }
        self.handler.set(None);
        self.pollables.borrow_mut().clear();
        self.acceptable_acceptors.take();
        self.clients_to_kill.take();
        self.readable_endpoints.take();
        self.flushable_endpoints.take();
        self.interest_update_endpoints.take();
        self.interest_update_acceptors.take();
        self.all_objects.borrow_mut().clear();
        // Ensure that the poll fd stays permanently readable.
        let _ = self.create_remote_destructor();
    }

    /// Creates a RAII destructor for this state.
    ///
    /// Dropping the destructor will automatically call [`State::destroy`] unless you
    /// first call [`Destructor::disable`].
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
            enabled: Cell::new(true),
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
            .register(id, r.as_fd())
            .map_err(StateErrorKind::PollError)?;
        let destroy = Arc::new(AtomicBool::new(false));
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Destructor(r, destroy.clone()));
        Ok(RemoteDestructor {
            destroy,
            _fd: w.into(),
            enabled: AtomicBool::new(true),
        })
    }
}

impl StateError {
    /// Returns whether this error was emitted because the state is already destroyed.
    ///
    /// This can be used to determine the severity of emitted log messages.
    pub fn is_destroyed(&self) -> bool {
        matches!(self.0, StateErrorKind::Destroyed)
    }
}

impl Drop for HandlerLock<'_> {
    fn drop(&mut self) {
        self.state.global_lock_held.set(false);
    }
}
