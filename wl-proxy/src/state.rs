use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        client::Client,
        connect_upstream,
        endpoint::{Endpoint, EndpointError},
        generated::wayland::wl_display::MetaWlDisplay,
        proxy::Proxy,
        trans::FlushResult,
    },
    error_reporter::Report,
    isnt::std_1::primitive::IsntSliceExt,
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        io,
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::{Errno, c},
};

#[derive(Debug, Error)]
pub enum StateError {
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
}

pub struct State {
    pub inner: Rc<InnerState>,
}

pub enum Pollable {
    Endpoint(EndpointWithClient),
    Acceptor(Rc<Acceptor>),
}

impl State {
    pub fn new() -> Result<Self, StateError> {
        let server = Endpoint::new(0, connect_upstream());
        server.idl.acquire();
        server.idl.acquire();
        let mut endpoints = HashMap::new();
        endpoints.insert(
            0,
            Pollable::Endpoint(EndpointWithClient {
                endpoint: server.clone(),
                client: None,
            }),
        );
        let epoll =
            uapi::epoll_create1(c::EPOLL_CLOEXEC).map_err(|e| StateError::CreateEpoll(e.into()))?;
        let state = Rc::new(InnerState {
            epoll,
            next_pollable_id: Cell::new(1),
            server,
            pollables: RefCell::new(endpoints),
            acceptable_acceptors: Default::default(),
            readable_endpoints: Default::default(),
            flushable_endpoints: Default::default(),
            interest_update_endpoints: Default::default(),
            interest_update_acceptors: Default::default(),
        });
        state.change_interest(&state.server, |i| i | c::EPOLLIN);
        state.register_socket(0, &state.server.socket)?;
        state.update_interests()?;
        let display = MetaWlDisplay::new(&state, 1);
        display
            .core()
            .set_server_id_unchecked(1, display.clone())
            .unwrap();
        let state = Self { inner: state };
        Ok(state)
    }

    pub fn dispatch_blocking(&self) -> Result<(), StateError> {
        self.inner.flush()?;
        self.inner.update_interests()?;
        if !self.inner.has_io_work() {
            self.inner.read_events(-1)?;
        }
        self.inner.accept_connections()?;
        self.inner.dispatch_pending()?;
        self.inner.flush()?;
        self.inner.update_interests()?;
        Ok(())
    }

    pub fn connect(&self) -> Result<(Rc<Client>, OwnedFd), StateError> {
        self.inner.connect()
    }

    pub fn create_acceptor(&self) -> Result<Rc<Acceptor>, StateError> {
        self.inner.create_acceptor()
    }
}

#[derive(Clone)]
pub struct EndpointWithClient {
    pub endpoint: Rc<Endpoint>,
    pub client: Option<Rc<Client>>,
}

pub struct InnerState {
    pub epoll: uapi::OwnedFd,
    pub next_pollable_id: Cell<u64>,
    pub server: Rc<Endpoint>,
    pub pollables: RefCell<HashMap<u64, Pollable>>,
    pub acceptable_acceptors: RefCell<Vec<Rc<Acceptor>>>,
    pub readable_endpoints: RefCell<Vec<EndpointWithClient>>,
    pub flushable_endpoints: RefCell<Vec<Rc<Endpoint>>>,
    pub interest_update_endpoints: RefCell<Vec<Rc<Endpoint>>>,
    pub interest_update_acceptors: RefCell<Vec<Rc<Acceptor>>>,
}

impl InnerState {
    pub(crate) fn handle_delete_id(&self, id: u32) {
        let proxy = self.server.objects.borrow_mut().remove(&id).unwrap();
        let core = proxy.core();
        core.server_obj_id.take();
        self.server.idl.release(id);
    }

    pub(crate) fn handle_error(&self, obj: Rc<dyn Proxy>, _code: u32, message: &str) {
        eprintln!("error: {}", message);
    }

    pub(crate) fn flush(&self) -> Result<(), StateError> {
        let endpoints = &mut *self.flushable_endpoints.borrow_mut();
        while let Some(endpoint) = endpoints.pop() {
            let res = match endpoint.flush() {
                Ok(r) => r,
                Err(_e) => {
                    todo!();
                }
            };
            match res {
                FlushResult::Done => {
                    endpoint.has_outgoing.set(false);
                    self.change_interest(&endpoint, |i| i & !c::EPOLLOUT);
                }
                FlushResult::Blocked => {
                    self.change_interest(&endpoint, |i| i | c::EPOLLOUT);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn accept_connections(self: &Rc<Self>) -> Result<(), StateError> {
        let acceptors = &mut *self.acceptable_acceptors.borrow_mut();
        let interest_update_acceptors = &mut *self.interest_update_acceptors.borrow_mut();
        while let Some(acceptor) = acceptors.pop() {
            interest_update_acceptors.push(acceptor.clone());
            for _ in 0..10 {
                let socket = acceptor.accept().map_err(StateError::AcceptConnection)?;
                let Some(socket) = socket else {
                    break;
                };
                self.create_client(socket)?;
            }
        }
        Ok(())
    }

    pub(crate) fn create_client(
        self: &Rc<Self>,
        socket: OwnedFd,
    ) -> Result<Rc<Client>, StateError> {
        let id = self.create_pollable_id();
        self.register_socket(id, &socket)?;
        let endpoint = Endpoint::new(id, socket);
        self.change_interest(&endpoint, |i| i | c::EPOLLIN);
        let display = MetaWlDisplay::new(self, 1);
        display.core().server_obj_id.set(Some(1));
        let client = Rc::new(Client {
            endpoint: endpoint.clone(),
            display,
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
        Ok(client)
    }

    pub(crate) fn dispatch_pending(&self) -> Result<(), StateError> {
        let endpoints = &mut *self.readable_endpoints.borrow_mut();
        while let Some(ewc) = endpoints.pop() {
            let res = ewc.endpoint.dispatch(ewc.client.as_ref());
            if let Err(e) = res {
                if let Some(_client) = &ewc.client {
                    panic!("{}", Report::new(e));
                } else {
                    return Err(StateError::DispatchEvents(e));
                }
            }
            self.change_interest(&ewc.endpoint, |i| i | c::EPOLLIN);
        }
        Ok(())
    }

    pub(crate) fn change_interest(
        &self,
        endpoint: &Rc<Endpoint>,
        f: impl FnOnce(c::c_int) -> c::c_int,
    ) {
        let old = endpoint.desired_interest.get();
        let new = f(old);
        endpoint.desired_interest.set(new);
        if old != new
            && endpoint.current_interest.get() != new
            && !endpoint.interest_update_queued.replace(true)
        {
            self.interest_update_endpoints
                .borrow_mut()
                .push(endpoint.clone());
        }
    }

    pub(crate) fn has_io_work(&self) -> bool {
        if self.acceptable_acceptors.borrow().is_not_empty() {
            return true;
        }
        if self.readable_endpoints.borrow().is_not_empty() {
            return true;
        }
        if self.flushable_endpoints.borrow().is_not_empty() {
            return true;
        }
        false
    }

    pub(crate) fn read_events(&self, mut timeout: c::c_int) -> Result<(), StateError> {
        let mut events = [c::epoll_event { events: 0, u64: 0 }; 16];
        let pollables = &*self.pollables.borrow();
        let flushable = &mut *self.flushable_endpoints.borrow_mut();
        let readable = &mut *self.readable_endpoints.borrow_mut();
        let acceptable = &mut *self.acceptable_acceptors.borrow_mut();
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
                        ewc.endpoint.current_interest.set(0);
                        let events = event.events as c::c_int;
                        self.change_interest(&ewc.endpoint, |i| i & !events);
                        if events & c::EPOLLIN != 0 {
                            readable.push(ewc.clone());
                        }
                        if events & c::EPOLLOUT != 0 {
                            flushable.push(ewc.endpoint.clone());
                        }
                    }
                    Pollable::Acceptor(a) => {
                        acceptable.push(a.clone());
                    }
                }
            }
        }
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
                Some(&event),
            )
            .map_err(|e| StateError::UpdateEpoll(io::Error::from_raw_os_error(e.0)))
        };
        let endpoints = &mut *self.interest_update_endpoints.borrow_mut();
        while let Some(endpoint) = endpoints.pop() {
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
        let acceptors = &mut *self.interest_update_acceptors.borrow_mut();
        while let Some(acceptor) = acceptors.pop() {
            let event = c::epoll_event {
                events: (c::EPOLLIN | c::EPOLLONESHOT) as _,
                u64: acceptor.id,
            };
            epoll_ctl(&acceptor.socket, &event)?;
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
        let client = self.create_client(server_fd.into())?;
        Ok((client, client_fd.into()))
    }

    pub fn create_acceptor(&self) -> Result<Rc<Acceptor>, StateError> {
        let id = self.create_pollable_id();
        let acceptor = Acceptor::new(id).map_err(StateError::CreateAcceptor)?;
        self.register_socket(id, &acceptor.socket)?;
        self.interest_update_acceptors
            .borrow_mut()
            .push(acceptor.clone());
        self.pollables
            .borrow_mut()
            .insert(id, Pollable::Acceptor(acceptor.clone()));
        Ok(acceptor)
    }
}
