use {
    crate::{
        baseline::Baseline,
        client::ClientHandler,
        object::{Object, ObjectCoreApi, ObjectRcUtils},
        protocols::{
            wayland::{
                wl_callback::WlCallback,
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
            wlproxy_test::{
                wlproxy_test::{WlproxyTest, WlproxyTestHandler},
                wlproxy_test_array_echo::WlproxyTestArrayEcho,
                wlproxy_test_fd_echo::WlproxyTestFdEcho,
                wlproxy_test_hops::WlproxyTestHops,
                wlproxy_test_non_forward::{WlproxyTestNonForward, WlproxyTestNonForwardHandler},
                wlproxy_test_object_echo::WlproxyTestObjectEcho,
                wlproxy_test_server_sent::{WlproxyTestServerSent, WlproxyTestServerSentHandler},
            },
        },
        state::State,
    },
    std::{os::fd::OwnedFd, rc::Rc, sync::mpsc, thread},
};

pub fn test_server(log: bool) -> Rc<OwnedFd> {
    let (send, recv) = mpsc::channel();
    thread::spawn(move || {
        let state = State::builder(Baseline::ALL_OF_THEM)
            .without_server()
            .with_logging(log)
            .with_log_prefix("server")
            .build()
            .unwrap();
        state.set_default_forward_to_server(false);
        let (client, fd) = state.connect().unwrap();
        send.send(fd).unwrap();
        client.set_handler(ClientHandlerImpl {
            state: state.clone(),
        });
        client.display().set_handler(DisplayHandler {});
        while state.is_not_destroyed() {
            state.dispatch_blocking().unwrap();
        }
    });
    Rc::new(recv.recv().unwrap())
}

struct ClientHandlerImpl {
    state: Rc<State>,
}

impl ClientHandler for ClientHandlerImpl {
    fn disconnected(self: Box<Self>) {
        self.state.destroy();
    }
}

struct DisplayHandler {}

impl WlDisplayHandler for DisplayHandler {
    fn handle_sync(&mut self, _slf: &Rc<WlDisplay>, callback: &Rc<WlCallback>) {
        callback.send_done(0);
        callback.delete_id();
    }

    fn handle_get_registry(&mut self, _slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(RegistryHandler {});
    }
}

struct RegistryHandler {}

impl WlRegistryHandler for RegistryHandler {
    fn handle_bind(&mut self, _slf: &Rc<WlRegistry>, _name: u32, id: Rc<dyn Object>) {
        let id = id.downcast::<WlproxyTest>();
        id.set_handler(TestHandler {});
    }
}

struct TestHandler {}

impl WlproxyTestHandler for TestHandler {
    fn handle_echo_array(
        &mut self,
        _slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestArrayEcho>,
        array: &[u8],
    ) {
        echo.send_array(array);
        echo.delete_id();
    }

    fn handle_echo_fd(
        &mut self,
        _slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestFdEcho>,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) {
        echo.send_fd(fd1, fd2);
        echo.delete_id();
    }

    fn handle_send_many_events(&mut self, slf: &Rc<WlproxyTest>) {
        for _ in 0..100_000 {
            slf.send_many_event();
        }
    }

    fn handle_count_hops(&mut self, _slf: &Rc<WlproxyTest>, id: &Rc<WlproxyTestHops>) {
        id.send_count(1);
        id.delete_id();
    }

    fn handle_echo_object(
        &mut self,
        _slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestObjectEcho>,
        object: Rc<dyn Object>,
    ) {
        echo.send_object(object);
        echo.delete_id();
    }

    fn handle_send_object(&mut self, slf: &Rc<WlproxyTest>) {
        let obj = slf.create_child();
        slf.send_sent_object(&obj);
        obj.set_handler(SentObjectHandler);
    }

    fn handle_create_non_forward(
        &mut self,
        _slf: &Rc<WlproxyTest>,
        id: &Rc<WlproxyTestNonForward>,
    ) {
        id.set_handler(NonForwardHandler);
    }
}

struct SentObjectHandler;

impl WlproxyTestServerSentHandler for SentObjectHandler {
    fn handle_send_destroy(&mut self, slf: &Rc<WlproxyTestServerSent>) {
        slf.send_destroyed();
    }
}

struct NonForwardHandler;

impl WlproxyTestNonForwardHandler for NonForwardHandler {
    fn handle_echo(&mut self, slf: &Rc<WlproxyTestNonForward>) {
        slf.send_echoed();
    }
}
