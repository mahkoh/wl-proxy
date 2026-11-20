use {
    crate::VeilError,
    linearize::StaticMap,
    std::{
        collections::{HashMap, HashSet},
        process::Command,
        rc::Rc,
        sync::Arc,
    },
    wl_proxy::{
        baseline::Baseline,
        object::{Object, ObjectCoreApi},
        protocols::{
            ObjectInterface,
            wayland::{
                wl_display::{WlDisplay, WlDisplayError, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(
    invert: bool,
    filter: HashMap<String, Option<u32>>,
    program: Vec<String>,
) -> Result<(), VeilError> {
    let server = SimpleProxy::new(Baseline::ALL_OF_THEM).map_err(VeilError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(VeilError::SpawnChild)?;
    let filter = create_filter(invert, filter);
    let err = server.run(|| WlDisplayHandlerImpl {
        filter: filter.clone(),
    });
    Err(VeilError::ServerFailed(err))
}

#[derive(Copy, Clone)]
enum Disposition {
    Forward,
    Hide,
    ReduceVersion(u32),
}

fn create_filter(
    invert: bool,
    filter: HashMap<String, Option<u32>>,
) -> Arc<StaticMap<ObjectInterface, Disposition>> {
    let default = match invert {
        true => Disposition::Hide,
        false => Disposition::Forward,
    };
    let mut res = StaticMap::from_fn(|_| default);
    for (interface, max_version) in filter {
        let Some(interface) = ObjectInterface::from_str(&interface) else {
            eprintln!("Unknown interface {}", interface);
            continue;
        };
        res[interface] = match max_version {
            None if invert => Disposition::Forward,
            None => Disposition::Hide,
            Some(v) => Disposition::ReduceVersion(v),
        };
    }
    Arc::new(res)
}

struct WlDisplayHandlerImpl {
    filter: Arc<StaticMap<ObjectInterface, Disposition>>,
}

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(WlRegistryHandlerImpl {
            filter: self.filter.clone(),
            announced: Default::default(),
        });
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl {
    filter: Arc<StaticMap<ObjectInterface, Disposition>>,
    // The names of the globals announced to the client. A client can bind
    // any name, so only binds of these are forwarded. A name stays here after
    // the global's removal, since the client can bind it until it has seen
    // the removal.
    announced: HashSet<u32>,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        if self.announced.contains(&name) {
            slf.send_bind(name, id);
            return;
        }
        let message = format!("invalid global {} ({})", id.interface().name(), name);
        if let Some(client) = slf.client() {
            let error = WlDisplayError::INVALID_OBJECT.0;
            client.display().send_error(slf.clone(), error, &message);
        }
    }

    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        match self.filter[interface] {
            Disposition::Forward => {
                self.announced.insert(name);
                slf.send_global(name, interface, version);
            }
            Disposition::Hide => {}
            Disposition::ReduceVersion(max) => {
                self.announced.insert(name);
                slf.send_global(name, interface, version.min(max));
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        if self.announced.contains(&name) {
            slf.send_global_remove(name);
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{WlDisplayHandlerImpl, create_filter},
        std::{
            cell::{Cell, RefCell},
            collections::HashMap,
            rc::Rc,
            time::Duration,
        },
        wl_proxy::{
            baseline::Baseline,
            object::Object,
            protocols::{
                ObjectInterface,
                wayland::{
                    wl_callback::{WlCallback, WlCallbackHandler},
                    wl_display::{WlDisplay, WlDisplayHandler},
                    wl_registry::{WlRegistry, WlRegistryHandler},
                    wl_seat::WlSeat,
                },
            },
            state::{State, StateHandler},
        },
    };

    // A compositor that announces wl_seat as global 1 and records binds.
    struct Compositor(Rc<Cell<bool>>);

    impl WlDisplayHandler for Compositor {
        fn handle_sync(&mut self, _slf: &Rc<WlDisplay>, callback: &Rc<WlCallback>) {
            callback.send_done(0);
        }

        fn handle_get_registry(&mut self, _slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
            registry.set_handler(Compositor(self.0.clone()));
            registry.send_global(1, ObjectInterface::WlSeat, 1);
        }
    }

    impl WlRegistryHandler for Compositor {
        fn handle_bind(&mut self, _slf: &Rc<WlRegistry>, _name: u32, _id: Rc<dyn Object>) {
            self.0.set(true);
        }
    }

    struct Error(Rc<RefCell<Option<String>>>);

    impl StateHandler for Error {
        fn display_error(
            self: Box<Self>,
            _object: Option<&Rc<dyn Object>>,
            _server_id: u32,
            _error: u32,
            msg: &str,
        ) {
            *self.0.borrow_mut() = Some(msg.to_string());
        }
    }

    struct Done(Rc<Cell<bool>>);

    impl WlCallbackHandler for Done {
        fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
            self.0.set(true);
        }
    }

    #[test]
    fn bind_of_hidden_global_is_refused() {
        let compositor = State::builder(Baseline::ALL_OF_THEM)
            .without_server()
            .build()
            .unwrap();
        let (compositor_client, compositor_fd) = compositor.connect().unwrap();
        let bound = Rc::new(Cell::new(false));
        compositor_client
            .display()
            .set_handler(Compositor(bound.clone()));

        let veil = State::builder(Baseline::ALL_OF_THEM)
            .with_server_fd(&Rc::new(compositor_fd))
            .build()
            .unwrap();
        let (veil_client, veil_fd) = veil.connect().unwrap();
        let filter = create_filter(false, HashMap::from([("wl_seat".to_string(), None)]));
        veil_client
            .display()
            .set_handler(WlDisplayHandlerImpl { filter });

        let client = State::builder(Baseline::ALL_OF_THEM)
            .with_server_fd(&Rc::new(veil_fd))
            .build()
            .unwrap();
        let error = Rc::new(RefCell::new(None));
        client.set_handler(Error(error.clone()));
        let display = client.display();
        // Bind the hidden global by its name before wl-veil has seen the
        // compositor's globals, then wait until the compositor has handled
        // the bind or the client has failed.
        let registry = display.new_send_get_registry();
        registry.send_bind(1, client.create_object::<WlSeat>(1));
        let done = Rc::new(Cell::new(false));
        display.new_send_sync().set_handler(Done(done.clone()));
        let timeout = Some(Duration::from_millis(1));
        while !done.get() && error.borrow().is_none() {
            compositor.dispatch(timeout).unwrap();
            veil.dispatch(timeout).unwrap();
            // The client fails on the error that refuses the bind.
            if let Err(e) = client.dispatch(timeout) {
                assert!(error.borrow().is_some(), "{e}");
            }
        }

        assert_eq!(
            error.borrow().as_deref(),
            Some("invalid global wl_seat (1)")
        );
        assert!(!bound.get());
    }
}
