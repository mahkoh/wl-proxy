use {
    crate::{
        baseline::Baseline,
        client::Client,
        object::{ConcreteObject, ObjectCoreApi, ObjectUtils},
        protocols::{
            ObjectInterface,
            wayland::{
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_display::WlDisplay,
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
        },
        state::{Destructor, State},
        test_framework::{install_logger, server::test_server},
    },
    std::{os::fd::AsRawFd, rc::Rc},
    uapi::c,
};

pub struct TestProxy {
    _proxy_destructor: Destructor,
    _client_destructor: Destructor,

    pub proxy_state: Rc<State>,
    pub _proxy_client: Rc<Client>,

    pub client_state: Rc<State>,
    pub client_display: Rc<WlDisplay>,
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
    let client_fd = Rc::new(client_fd);
    let client_state = State::builder(Baseline::ALL_OF_THEM)
        .with_server_fd(&client_fd)
        .with_logging(log)
        .with_log_prefix("client")
        .build()
        .unwrap();
    client_state.set_default_forward_to_client(false);
    TestProxy {
        _proxy_destructor: proxy_state.create_destructor(),
        _client_destructor: client_state.create_destructor(),
        proxy_state,
        _proxy_client: client,
        client_display: client_state.display(),
        client_state,
    }
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
        let mut did_work = false;
        did_work |= self.client_state.dispatch_available().unwrap();
        did_work |= self.proxy_state.dispatch_available().unwrap();
        if did_work {
            return;
        }
        self.client_state.before_poll().unwrap();
        self.proxy_state.before_poll().unwrap();
        let mut pollfd = [
            c::pollfd {
                fd: self.proxy_state.poll_fd().as_raw_fd(),
                events: c::POLLIN,
                revents: 0,
            },
            c::pollfd {
                fd: self.client_state.poll_fd().as_raw_fd(),
                events: c::POLLIN,
                revents: 0,
            },
        ];
        uapi::poll(&mut pollfd, -1).unwrap();
    }

    pub fn get_global<T: ConcreteObject>(&self) -> Rc<T> {
        let registry = self.client_display.new_send_get_registry();
        struct Rh<T> {
            t: Option<Rc<T>>,
        }
        impl<T> WlRegistryHandler for Rh<T>
        where
            T: ConcreteObject,
        {
            fn handle_global(
                &mut self,
                slf: &Rc<WlRegistry>,
                name: u32,
                interface: ObjectInterface,
                version: u32,
            ) {
                if interface == T::INTERFACE {
                    let obj = slf.state().create_object::<T>(version);
                    self.t = Some(obj.clone());
                    slf.send_bind(name, obj);
                }
            }
        }
        registry.set_handler(Rh::<T> { t: None });
        let obj = loop {
            if let Some(t) = registry.get_handler_mut::<Rh<T>>().t.take() {
                break t;
            }
            self.dispatch_blocking();
        };
        obj
    }
}
