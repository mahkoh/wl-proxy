//! Helpers that take care of most of the boilerplate for simple proxies.

use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        baseline::Baseline,
        client::ClientHandler,
        protocols::wayland::wl_display::WlDisplayHandler,
        state::{Destructor, RemoteDestructor, State, StateError},
        utils::env::{WAYLAND_DISPLAY, WAYLAND_SOCKET},
    },
    error_reporter::Report,
    parking_lot::Mutex,
    run_on_drop::on_drop,
    std::{
        io,
        os::{
            fd::{AsRawFd, OwnedFd},
            unix::prelude::ExitStatusExt,
        },
        process::{Command, exit},
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering::Relaxed},
        thread,
        time::Duration,
    },
    thiserror::Error,
    uapi::{c, raise},
};

enum ProxyMode {
    WaylandDisplay(Rc<Acceptor>),
    WaylandSocket {
        compositor_connection: OwnedFd,
        client_connection: OwnedFd,
        client_opposite: OwnedFd,
    },
}

/// A simple proxy server that spawns a thread for each client.
///
/// This server will create an acceptor and create a [`State`] for
/// each client that connects to the acceptor.
pub struct SimpleProxy {
    baseline: Baseline,
    mode: ProxyMode,
}

/// An error returned by a [`SimpleProxy`].
#[derive(Debug, Error)]
#[error(transparent)]
pub struct SimpleProxyError(#[from] SimpleProxyErrorKind);

#[derive(Debug, Error)]
enum SimpleProxyErrorKind {
    #[error("could not create a socketpair")]
    CreateSocketPair(#[source] io::Error),
    #[error("could not create an acceptor")]
    CreateAcceptor(#[source] AcceptorError),
    #[error("could not accept a connection")]
    AcceptConnection(#[source] AcceptorError),
    #[error("could not spawn a thread")]
    SpawnThread(#[source] io::Error),
}

fn run_client_thread<H>(
    id: usize,
    state: Result<Rc<State>, StateError>,
    client_socket: OwnedFd,
    display_handler: impl Fn() -> H + Sync,
    destructors: &Mutex<Option<Vec<RemoteDestructor>>>,
) where
    H: WlDisplayHandler,
{
    let state = match state {
        Ok(s) => s,
        Err(e) => {
            log::error!("Could not create a new state: {}", Report::new(e));
            return;
        }
    };
    match state.create_remote_destructor() {
        Ok(d) => match &mut *destructors.lock() {
            Some(des) => des.push(d),
            _ => return,
        },
        Err(e) => {
            log::error!("Could not create a remote destructor: {}", Report::new(e),);
            return;
        }
    }
    let client = match state.add_client(&Rc::new(client_socket)) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Could not add client to state: {}", Report::new(e));
            return;
        }
    };
    client.set_handler(ClientHandlerImpl {
        id,
        _destructor: state.create_destructor(),
    });
    let handler = display_handler();
    client.display().set_handler(handler);
    while state.is_not_destroyed() {
        if let Err(e) = state.dispatch_blocking() {
            log::error!("Could not dispatch state: {}", Report::new(e));
        }
    }
}

impl SimpleProxy {
    /// Creates a new [`SimpleProxy`].
    pub fn new(
        baseline: Baseline,
        wayland_socket: Option<OwnedFd>,
    ) -> Result<SimpleProxy, SimpleProxyError> {
        Ok(Self {
            baseline,
            mode: if let Some(compositor_connection) = wayland_socket {
                let (client_connection, client_opposite) = uapi::socketpair(
                    c::AF_UNIX,
                    c::SOCK_STREAM | c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
                    0,
                )
                .map_err(|e| SimpleProxyErrorKind::CreateSocketPair(e.into()))?;

                uapi::fcntl_setfd(client_opposite.as_raw_fd(), 0)
                    .map_err(|e| SimpleProxyErrorKind::CreateSocketPair(e.into()))?;

                ProxyMode::WaylandSocket {
                    compositor_connection,
                    client_connection: client_connection.into(),
                    client_opposite: client_opposite.into(),
                }
            } else {
                ProxyMode::WaylandDisplay(
                    Acceptor::new(1000, false).map_err(SimpleProxyErrorKind::CreateAcceptor)?,
                )
            },
        })
    }

    /// Returns the name of the display used by this proxy, if one is used and WAYLAND_SOCKET
    /// is not provided.
    ///
    /// The `WAYLAND_DISPLAY` environment variable should be set to this value for clients
    /// that should connect to this proxy. See [`SimpleCommandExt::with_wayland_display`].
    pub fn display(&self) -> Option<&str> {
        match &self.mode {
            ProxyMode::WaylandDisplay(acceptor) => Some(acceptor.display()),
            ProxyMode::WaylandSocket { .. } => None,
        }
    }

    /// Returns the file descriptor id used by this proxy, if one is used and WAYLAND_SOCKET
    /// is not provided.
    ///
    /// The `WAYLAND_SOCKET` environment variable should be set to this value the client
    /// that should connect to this proxy. See [`SimpleCommandExt::with_wayland_socket`].
    pub fn socket(&self) -> Option<i32> {
        match &self.mode {
            ProxyMode::WaylandDisplay(_) => None,
            ProxyMode::WaylandSocket {
                client_opposite, ..
            } => Some(client_opposite.as_raw_fd()),
        }
    }

    /// Runs the proxy indefinitely.
    ///
    /// This function does not return unless a fatal error occurs.
    pub fn run<H>(self, display_handler: impl Fn() -> H + Sync) -> SimpleProxyError
    where
        H: WlDisplayHandler,
    {
        static ID: AtomicUsize = AtomicUsize::new(1);
        let display_handler = &display_handler;
        let destructors = Mutex::new(Some(vec![]));
        let destructors = &destructors;
        match self.mode {
            ProxyMode::WaylandDisplay(acceptor) => SimpleProxyError(thread::scope(|s| {
                let _stop_all_proxies = on_drop(|| *destructors.lock() = None);
                loop {
                    let socket = match acceptor.accept() {
                        Ok(s) => s.expect("blocking acceptor returned None"),
                        Err(e) => return SimpleProxyErrorKind::AcceptConnection(e),
                    };
                    let id = ID.fetch_add(1, Relaxed);
                    let name = format!("socket-{id}");
                    log::debug!("Client {id} connected");
                    let res =
                        thread::Builder::new()
                            .name(name.clone())
                            .spawn_scoped(s, move || {
                                let state =
                                    State::builder(self.baseline).with_log_prefix(&name).build();
                                run_client_thread(id, state, socket, display_handler, destructors);
                            });
                    if let Err(e) = res {
                        return SimpleProxyErrorKind::SpawnThread(e);
                    }
                }
            })),
            ProxyMode::WaylandSocket {
                compositor_connection,
                client_connection,
                client_opposite,
            } => {
                drop(client_opposite);

                let state = State::builder(self.baseline)
                    .with_log_prefix("socket")
                    .with_server_fd(&Rc::new(compositor_connection))
                    .build();
                let id = ID.fetch_add(1, Relaxed);
                run_client_thread(id, state, client_connection, display_handler, destructors);

                loop {
                    std::thread::sleep(Duration::MAX);
                }
            }
        }
    }
}

struct ClientHandlerImpl {
    id: usize,
    _destructor: Destructor,
}

impl ClientHandler for ClientHandlerImpl {
    fn disconnected(self: Box<Self>) {
        log::debug!("Client {} disconnected", self.id);
    }
}

/// Extensions for [`Command`].
pub trait SimpleCommandExt {
    /// Sets the `WAYLAND_DISPLAY` environment variable if `display` is Some.
    fn with_optional_wayland_display(&mut self, display: Option<&str>) -> &mut Command {
        if let Some(disp) = display {
            self.with_wayland_display(disp)
        } else {
            self.without_wayland_display()
        }
    }
    /// Sets the `WAYLAND_DISPLAY` environment variable if `socket` is Some.
    fn with_optional_wayland_socket(&mut self, socket: Option<i32>) -> &mut Command {
        if let Some(sock) = socket {
            self.with_wayland_socket(sock)
        } else {
            self.without_wayland_socket()
        }
    }
    /// Sets the `WAYLAND_DISPLAY` environment variable.
    fn with_wayland_display(&mut self, display: &str) -> &mut Command;
    /// Sets the `WAYLAND_SOCKET` environment variable.
    fn with_wayland_socket(&mut self, socket: i32) -> &mut Command;
    /// Clears any `WAYLAND_DISPLAY` environment variable.
    fn without_wayland_display(&mut self) -> &mut Command;
    /// Clears any `WAYLAND_SOCKET` environment variable.
    fn without_wayland_socket(&mut self) -> &mut Command;
    /// Spawns the application, waits for it to exit, and then calls [`exit`] with the
    /// same exit code.
    fn spawn_and_forward_exit_code(&mut self) -> Result<(), io::Error>;
}

impl SimpleCommandExt for Command {
    fn with_wayland_display(&mut self, display: &str) -> &mut Command {
        self.env(WAYLAND_DISPLAY, display)
    }

    fn with_wayland_socket(&mut self, socket: i32) -> &mut Command {
        self.env(WAYLAND_SOCKET, socket.to_string())
    }

    fn without_wayland_display(&mut self) -> &mut Command {
        self.env_remove(WAYLAND_DISPLAY)
    }

    fn without_wayland_socket(&mut self) -> &mut Command {
        self.env_remove(WAYLAND_SOCKET)
    }

    fn spawn_and_forward_exit_code(&mut self) -> Result<(), io::Error> {
        let mut child = self.spawn()?;
        thread::spawn(move || match child.wait() {
            Ok(e) => {
                if let Some(code) = e.code() {
                    exit(code);
                }
                if let Some(signal) = e.signal() {
                    let _ = raise(signal);
                    exit(1);
                }
                eprintln!("Child terminated with neither a signal nor an exit code");
                exit(1);
            }
            Err(e) => {
                eprintln!("Could not wait for child: {}", Report::new(e));
                exit(1);
            }
        });
        Ok(())
    }
}
