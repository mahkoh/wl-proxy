//! Helpers that take care of most of the boilerplate for simple proxies.

use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        baseline::Baseline,
        client::ClientHandler,
        protocols::wayland::wl_display::WlDisplayHandler,
        state::{Destructor, State},
        utils::env::WAYLAND_DISPLAY,
    },
    error_reporter::Report,
    parking_lot::Mutex,
    run_on_drop::on_drop,
    std::{
        io,
        os::unix::prelude::ExitStatusExt,
        process::{Command, exit},
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering::Relaxed},
        thread,
    },
    thiserror::Error,
    uapi::raise,
};

/// A simple proxy server that spawns a thread for each client.
///
/// This server will create an acceptor and create a [`State`] for
/// each client that connects to the acceptor.
pub struct SimpleProxy {
    baseline: Baseline,
    acceptor: Rc<Acceptor>,
}

/// An error returned by a [`SimpleProxy`].
#[derive(Debug, Error)]
#[error(transparent)]
pub struct SimpleProxyError(#[from] SimpleProxyErrorKind);

#[derive(Debug, Error)]
enum SimpleProxyErrorKind {
    #[error("could not create an acceptor")]
    CreateAcceptor(#[source] AcceptorError),
    #[error("could not accept a connection")]
    AcceptConnection(#[source] AcceptorError),
    #[error("could not spawn a thread")]
    SpawnThread(#[source] io::Error),
}

impl SimpleProxy {
    /// Creates a new [`SimpleProxy`].
    pub fn new(baseline: Baseline) -> Result<SimpleProxy, SimpleProxyError> {
        Ok(Self {
            baseline,
            acceptor: Acceptor::new(1000, false).map_err(SimpleProxyErrorKind::CreateAcceptor)?,
        })
    }

    /// Returns the name of the display used by this proxy.
    ///
    /// The `WAYLAND_DISPLAY` environment variable should be set to this value for clients
    /// that should connect to this proxy. See [`SimpleCommandExt::with_wayland_display`].
    pub fn display(&self) -> &str {
        self.acceptor.display()
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
        let err = thread::scope(|s| {
            let _stop_all_proxies = on_drop(|| *destructors.lock() = None);
            loop {
                let socket = match self.acceptor.accept() {
                    Ok(s) => s.expect("blocking acceptor returned None"),
                    Err(e) => return SimpleProxyErrorKind::AcceptConnection(e),
                };
                let id = ID.fetch_add(1, Relaxed);
                let name = format!("socket-{id}");
                log::debug!("Client {id} connected");
                let res = thread::Builder::new()
                    .name(name.clone())
                    .spawn_scoped(s, move || {
                        let state = State::builder(self.baseline).with_log_prefix(&name).build();
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
                                log::error!(
                                    "Could not create a remote destructor: {}",
                                    Report::new(e),
                                );
                                return;
                            }
                        }
                        let client = match state.add_client(&Rc::new(socket)) {
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
                    });
                if let Err(e) = res {
                    return SimpleProxyErrorKind::SpawnThread(e);
                }
            }
        });
        SimpleProxyError(err)
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
    /// Sets the `WAYLAND_DISPLAY` environment variable.
    fn with_wayland_display(&mut self, display: &str) -> &mut Command;
    /// Spawns the application, waits for it to exit, and then calls [`exit`] with the
    /// same exit code.
    fn spawn_and_forward_exit_code(&mut self) -> Result<(), io::Error>;
}

impl SimpleCommandExt for Command {
    fn with_wayland_display(&mut self, display: &str) -> &mut Command {
        self.env(WAYLAND_DISPLAY, display)
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
