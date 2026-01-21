use {
    crate::{
        baseline::Baseline,
        client::{Client, ClientHandler},
        object::{Object, ObjectRcUtils, ObjectUtils},
        protocols::{
            wayland::{
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
            wlproxy_test::wlproxy_test::WlproxyTest,
        },
        state::{Destructor, State, StateError},
        test_framework::{install_logger, server::test_server},
    },
    std::{
        cell::{Cell, RefCell},
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
    uapi::c,
};

pub struct TestProxy {
    pub _proxy_destructor: Destructor,
    pub _client_destructor: Destructor,

    pub proxy_state: Rc<State>,
    pub proxy_client: Rc<Client>,
    pub proxy_test: Rc<WlproxyTest>,

    pub client_state: Rc<State>,
    pub client_display: Rc<WlDisplay>,
    pub client_test: Rc<WlproxyTest>,
    pub client_fd: Rc<OwnedFd>,
}

pub fn test_proxy() -> TestProxy {
    install_logger();
    test_proxy_(true)
}

pub fn test_proxy_no_log() -> TestProxy {
    test_proxy_(false)
}

fn test_proxy_(log: bool) -> TestProxy {
    install_logger();
    let server = test_server(log);
    let proxy_state = State::builder(Baseline::ALL_OF_THEM)
        .with_server_fd(&server)
        .with_logging(log)
        .with_log_prefix("proxy ")
        .build()
        .unwrap();
    let (client, client_fd) = proxy_state.connect().unwrap();
    struct Handler(Rc<RefCell<Option<Rc<WlproxyTest>>>>);
    impl WlDisplayHandler for Handler {
        fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
            registry.set_handler(Handler(self.0.clone()));
            slf.send_get_registry(registry);
        }
    }
    impl WlRegistryHandler for Handler {
        fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
            *self.0.borrow_mut() = Some(id.downcast());
            slf.send_bind(name, id);
        }
    }
    let proxy_test = Rc::new(RefCell::new(None));
    client.display.set_handler(Handler(proxy_test.clone()));
    let client_fd = Rc::new(client_fd);
    let client_state = State::builder(Baseline::ALL_OF_THEM)
        .with_server_fd(&client_fd)
        .with_logging(log)
        .with_log_prefix("client")
        .build()
        .unwrap();
    client_state.set_default_forward_to_client(false);
    let client_display = client_state.display();
    let registry = client_display.new_send_get_registry();
    let client_test = client_state.create_object::<WlproxyTest>(1);
    registry.send_bind(0, client_test.clone());
    let proxy_test = loop {
        if let Some(obj) = proxy_test.borrow_mut().take() {
            break obj;
        }
        dispatch_blocking([&client_state, &proxy_state]).unwrap();
    };
    TestProxy {
        _proxy_destructor: proxy_state.create_destructor(),
        _client_destructor: client_state.create_destructor(),
        proxy_state,
        proxy_client: client,
        client_display,
        client_state,
        proxy_test,
        client_test,
        client_fd,
    }
}

pub fn dispatch_blocking<const N: usize>(states: [&Rc<State>; N]) -> Result<(), StateError> {
    let mut did_work = false;
    for state in states {
        did_work |= state.dispatch_available()?;
    }
    if did_work {
        return Ok(());
    }
    for state in states {
        state.before_poll().unwrap();
    }
    let mut pollfd = states.map(|s| c::pollfd {
        fd: s.poll_fd().as_raw_fd(),
        events: c::POLLIN,
        revents: 0,
    });
    uapi::poll(&mut pollfd, -1).unwrap();
    Ok(())
}

impl TestProxy {
    pub fn sync(&self) {
        let wl_callback = self.client_display.new_send_sync();
        struct CallbackHandler {
            done: bool,
        }
        impl WlCallbackHandler for CallbackHandler {
            fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
                self.done = true;
            }
        }
        wl_callback.set_handler(CallbackHandler { done: false });
        while !wl_callback.get_handler_mut::<CallbackHandler>().done {
            self.dispatch_blocking();
        }
    }

    pub fn dispatch_blocking(&self) {
        dispatch_blocking([&self.client_state, &self.proxy_state]).unwrap();
    }

    pub fn await_client_disconnected(&self) {
        struct H(Rc<Cell<bool>>);
        impl ClientHandler for H {
            fn disconnected(self: Box<Self>) {
                self.0.set(true);
            }
        }
        let disconnected = Rc::new(Cell::new(false));
        self.proxy_client.set_handler(H(disconnected.clone()));
        assert!(!disconnected.get());
        while !disconnected.get() {
            self.dispatch_blocking();
        }
    }
}
