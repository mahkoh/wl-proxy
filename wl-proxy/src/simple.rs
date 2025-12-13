use {
    crate::{
        acceptor::{Acceptor, AcceptorError},
        baseline::Baseline,
        client::ClientHandler,
        protocols::wayland::wl_display::WlDisplayHandler,
        state::{Destructor, StateBuilder},
        utils::env::WAYLAND_DISPLAY,
    },
    error_reporter::Report,
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

pub struct SimpleServer {
    baseline: Baseline,
    acceptor: Rc<Acceptor>,
}

#[derive(Debug, Error)]
pub enum SimpleServerError {
    #[error("could not create an acceptor")]
    CreateAcceptor(#[source] AcceptorError),
    #[error("could not accept a connection")]
    AcceptConnection(#[source] AcceptorError),
    #[error("could not spawn a thread")]
    SpawnThread(#[source] io::Error),
}

impl SimpleServer {
    pub fn new(baseline: Baseline) -> Result<SimpleServer, SimpleServerError> {
        Ok(Self {
            baseline,
            acceptor: Acceptor::new(1000, false).map_err(SimpleServerError::CreateAcceptor)?,
        })
    }

    pub fn display(&self) -> &str {
        self.acceptor.display()
    }

    pub fn run<H>(self, display_handler: impl Fn() -> H + Sync) -> SimpleServerError
    where
        H: WlDisplayHandler + 'static,
    {
        static ID: AtomicUsize = AtomicUsize::new(1);
        let display_handler = &display_handler;
        thread::scope(|s| {
            loop {
                let socket = match self.acceptor.accept() {
                    Ok(s) => s.expect("blocking acceptor returned None"),
                    Err(e) => return SimpleServerError::AcceptConnection(e),
                };
                let id = ID.fetch_add(1, Relaxed);
                let name = format!("socket-{id}");
                log::debug!("client {id} connected");
                let res = thread::Builder::new()
                    .name(name.clone())
                    .spawn_scoped(s, move || {
                        let state = StateBuilder::new(self.baseline)
                            .with_log_prefix(&name)
                            .build();
                        let state = match state {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("could not create a new state: {}", Report::new(e));
                                return;
                            }
                        };
                        let client = match state.add_client(&Rc::new(socket)) {
                            Ok(c) => c,
                            Err(e) => {
                                log::error!("could not add client to state: {}", Report::new(e));
                                return;
                            }
                        };
                        client.set_handler(ClientHandlerImpl {
                            id,
                            _destructor: state.create_destructor(),
                        });
                        let handler = display_handler();
                        client.display.set_handler(handler);
                        while state.is_not_destroyed() {
                            if let Err(e) = state.dispatch_blocking() {
                                log::error!("could not dispatch state: {}", e);
                            }
                        }
                    });
                if let Err(e) = res {
                    return SimpleServerError::SpawnThread(e);
                }
            }
        })
    }
}

struct ClientHandlerImpl {
    id: usize,
    _destructor: Destructor,
}

impl ClientHandler for ClientHandlerImpl {
    fn disconnected(self: Box<Self>) {
        log::debug!("client {} disconnected", self.id);
    }
}

pub trait SimpleCommandExt {
    fn with_wayland_display(&mut self, display: &str) -> &mut Command;
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
                eprintln!("child terminated with neither a signal nor an exit code");
                exit(1);
            }
            Err(e) => {
                eprintln!("could not wait for child: {}", Report::new(e));
                exit(1);
            }
        });
        Ok(())
    }
}
