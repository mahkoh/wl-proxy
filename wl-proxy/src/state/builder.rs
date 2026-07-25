use {
    crate::{
        baseline::Baseline,
        endpoint::Endpoint,
        object::{Object, ObjectPrivate},
        poll::{self, Poller},
        protocols::wayland::wl_display::WlDisplay,
        state::{EndpointWithClient, Pollable, State, StateError, StateErrorKind},
        utils::env::{WAYLAND_DISPLAY, WL_PROXY_DEBUG, XDG_RUNTIME_DIR},
    },
    linearize::Linearize,
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        env::var,
        os::fd::{AsFd, OwnedFd},
        rc::Rc,
    },
    uapi::c::{self, sockaddr_un},
};

/// A builder for a [`State`].
///
/// This type can be constructed with [`State::builder`].
pub struct StateBuilder {
    baseline: Baseline,
    server: Option<Server>,
    log: bool,
    log_prefix: String,
}

enum Server {
    None,
    Fd(Rc<OwnedFd>),
    DisplayName(String),
}

#[derive(Copy, Clone, Linearize)]
pub(crate) enum StaticPollableIds {
    Server,
    Unsuspend,
}

impl StateBuilder {
    pub(super) fn new(baseline: Baseline) -> Self {
        Self {
            baseline,
            server: Default::default(),
            log: var(WL_PROXY_DEBUG).as_deref() == Ok("1"),
            log_prefix: Default::default(),
        }
    }

    /// Builds the state.
    ///
    /// The server to connect to is chosen as follows:
    ///
    /// - If [`Self::with_server_fd`] was used, that FD is used. To support WAYLAND_SOCKET,
    ///   callers must explicitly provide the server FD.
    /// - Otherwise, if [`Self::with_server_display_name`] was used, that display name is
    ///   used.
    /// - Otherwise, the display name from the `WAYLAND_DISPLAY` environment variable is
    ///   used.
    pub fn build(self) -> Result<Rc<State>, StateError> {
        let server_fd = 'fd: {
            let display_name = match self.server {
                None => None,
                Some(Server::None) => break 'fd None,
                Some(Server::Fd(fd)) => break 'fd Some(fd),
                Some(Server::DisplayName(n)) => Some(n),
            };
            let mut name = match display_name {
                Some(n) => n,
                _ => var(WAYLAND_DISPLAY)
                    .ok()
                    .ok_or(StateErrorKind::WaylandDisplay)?,
            };
            if name.is_empty() {
                return Err(StateErrorKind::WaylandDisplayEmpty.into());
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
            Some(Rc::new(socket.into()))
        };
        let mut endpoints = HashMap::new();
        let mut server = None;
        if let Some(server_fd) = &server_fd {
            let s = Endpoint::new(StaticPollableIds::Server as u64, server_fd);
            s.idl.acquire();
            s.idl.acquire();
            endpoints.insert(
                StaticPollableIds::Server as u64,
                Pollable::Endpoint(EndpointWithClient {
                    endpoint: s.clone(),
                    client: None,
                }),
            );
            server = Some(s);
        }
        let unsuspend_fd = uapi::eventfd(0, c::EFD_CLOEXEC | c::EFD_NONBLOCK)
            .map(Into::into)
            .map_err(|e| StateErrorKind::CreateEventfd(e.into()))?;
        endpoints.insert(StaticPollableIds::Unsuspend as u64, Pollable::Unsuspend);
        let poller = Poller::new().map_err(StateErrorKind::PollError)?;
        #[cfg(feature = "logging")]
        let log_prefix = {
            use {crate::utils::env::WL_PROXY_PREFIX, isnt::std_1::string::IsntStringExt};
            let mut log_prefix = String::new();
            if let Ok(prefix) = var(WL_PROXY_PREFIX) {
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
            log_prefix
        };
        let state = Rc::new(State {
            baseline: self.baseline,
            poller,
            next_pollable_id: Cell::new(StaticPollableIds::LENGTH as u64),
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
            all_objects: Default::default(),
            next_object_id: Cell::new(1),
            #[cfg(feature = "logging")]
            log: self.log,
            #[cfg(feature = "logging")]
            log_prefix,
            #[cfg(feature = "logging")]
            log_writer: RefCell::new(std::io::BufWriter::with_capacity(
                1024,
                uapi::Fd::new(c::STDERR_FILENO),
            )),
            global_lock_held: Default::default(),
            object_stash: Default::default(),
            forward_to_client: Cell::new(true),
            forward_to_server: Cell::new(true),
            unsuspend_fd,
            unsuspend_requests: Default::default(),
            has_unsuspend_requests: Default::default(),
            unsuspend_triggered: Default::default(),
        });
        if let Some(server) = &state.server {
            state.change_interest(server, |i| i | poll::READABLE);
            state
                .poller
                .register(server.id, server.socket.as_fd())
                .map_err(StateErrorKind::PollError)?;
            let display = WlDisplay::new(&state, 1);
            display
                .core()
                .set_server_id_unchecked(1, display.clone())
                .unwrap();
        }
        state
            .poller
            .register_edge_triggered(
                StaticPollableIds::Unsuspend as u64,
                state.unsuspend_fd.as_fd(),
                poll::READABLE,
            )
            .map_err(StateErrorKind::PollError)?;
        Ok(state)
    }

    /// Constructs a state without a server.
    pub fn without_server(mut self) -> Self {
        self.server = Some(Server::None);
        self
    }

    /// Sets the server file descriptor to connect to.
    pub fn with_server_fd(mut self, fd: &Rc<OwnedFd>) -> Self {
        self.server = Some(Server::Fd(fd.clone()));
        self
    }

    /// Sets the server file descriptor to connect to.
    pub fn with_server_opt_fd(mut self, fd: &Option<Rc<OwnedFd>>) -> Self {
        if let Some(rfd) = fd {
            self.server = Some(Server::Fd(rfd.clone()));
        } else {
            self.server = None;
        }
        self
    }

    /// Sets the server display name to connect to.
    pub fn with_server_display_name(mut self, name: &str) -> Self {
        self.server = Some(Server::DisplayName(name.to_owned()));
        self
    }

    /// Enables or disables logging.
    ///
    /// If this function is not used, then logging is enabled if and only if the
    /// `WL_PROXY_DEBUG` environment variable is set to `1`.
    pub fn with_logging(mut self, log: bool) -> Self {
        self.log = log;
        self
    }

    /// Sets a log prefix for messages emitted by this state.
    pub fn with_log_prefix(mut self, prefix: &str) -> Self {
        self.log_prefix = prefix.to_string();
        self
    }
}
