use {
    crate::{
        baseline::Baseline,
        object::{Object, ObjectCoreApi, ObjectRcUtils, ObjectUtils},
        protocols::{
            wayland::wl_keyboard::WlKeyboard,
            wlproxy_test::{
                wlproxy_test::{WlproxyTest, WlproxyTestHandler},
                wlproxy_test_non_forward::{WlproxyTestNonForward, WlproxyTestNonForwardHandler},
                wlproxy_test_server_sent::{WlproxyTestServerSent, WlproxyTestServerSentHandler},
            },
        },
        state::State,
        test_framework::proxy::test_proxy,
    },
    std::rc::Rc,
};

#[test]
fn server_sent() {
    let tp = test_proxy();

    {
        struct H;
        impl WlproxyTestHandler for H {
            fn handle_sent_object(
                &mut self,
                slf: &Rc<WlproxyTest>,
                echo: &Rc<WlproxyTestServerSent>,
            ) {
                echo.set_handler(H);
                slf.send_sent_object(echo);
            }
        }
        impl WlproxyTestServerSentHandler for H {
            fn handle_destroyed(&mut self, slf: &Rc<WlproxyTestServerSent>) {
                // We need to implement this manually since the default handler does not
                // forward destructor events for server-sent objects. Such events are
                // illegal in the wayland spec.
                slf.send_destroyed();
            }
        }
        tp.client.proxy_test.set_handler(H);
    }

    let (server, client) = tp.get_server_sent_object();

    {
        struct H;
        impl WlproxyTestServerSentHandler for H {
            fn handle_destroyed(&mut self, slf: &Rc<WlproxyTestServerSent>) {
                // We need to implement this manually since the default handler does not
                // forward destructor events for server-sent objects. Such events are
                // illegal in the wayland spec.
                slf.send_destroyed();
            }
        }
        server.set_handler(H);
    }

    struct S(bool);
    impl WlproxyTestServerSentHandler for S {
        fn handle_destroyed(&mut self, _slf: &Rc<WlproxyTestServerSent>) {
            self.0 = true;
        }
    }
    client.set_handler(S(false));
    client.send_send_destroy();
    tp.sync();
    assert!(client.get_handler_mut::<S>().0);
}

#[test]
#[should_panic(expected = "wl_display")]
fn wrong_downcast() {
    let tp = test_proxy();
    (tp.client.display.clone() as Rc<dyn Object>).downcast::<WlKeyboard>();
}

#[test]
fn double_send() {
    let tp = test_proxy();
    let sync = tp.client.display.new_send_sync();
    assert!(tp.client.display.try_send_sync(&sync).is_err());
    tp.sync();
}

#[test]
fn request_without_server() {
    let state = State::builder(Baseline::ALL_OF_THEM)
        .without_server()
        .build()
        .unwrap();
    assert!(state.display().new_try_send_sync().is_err());
}

#[test]
fn duplicate_client_id() {
    let tp = test_proxy();
    let dummy = tp.client.test.new_send_create_dummy();
    tp.client
        .state
        .server
        .as_ref()
        .unwrap()
        .idl
        .release(dummy.core().server_obj_id.take().unwrap());
    tp.client.test.send_create_dummy(&dummy);
    tp.await_client_disconnected();
}

#[test]
fn client_object_with_server_id() {
    let tp = test_proxy();
    tp.client
        .state
        .server
        .as_ref()
        .unwrap()
        .outgoing
        .borrow_mut()
        .formatter()
        .words([1, 0, !0]);
    tp.client.display.new_send_sync();
    tp.await_client_disconnected();
}

#[test]
fn duplicate_generated_client_id() {
    let tp = test_proxy();
    let ss = tp.client.proxy_test.new_send_sent_object();
    assert!(tp.client.proxy_test.try_send_sent_object(&ss).is_err());
}

#[test]
fn destroyed_client() {
    let tp = test_proxy();
    tp.client.proxy_client.disconnect();
    assert!(tp.client.proxy_test.new_try_send_sent_object().is_err());
}

#[test]
#[should_panic(expected = "NotServerId(50)")]
fn invalid_server_id() {
    let tp = test_proxy();
    tp.client
        .proxy_client
        .endpoint
        .outgoing
        .borrow_mut()
        .formatter()
        .words([
            tp.client.proxy_test.client_id().unwrap(),
            1,  // event sent_object
            50, // id
        ]);
    tp.sync();
}

#[test]
#[should_panic(expected = "ServerIdInUse(4294967295)")]
fn duplicate_server_id() {
    let tp = test_proxy();
    for _ in 0..2 {
        tp.client
            .proxy_client
            .endpoint
            .outgoing
            .borrow_mut()
            .formatter()
            .words([
                tp.client.proxy_test.client_id().unwrap(),
                1,  // event sent_object
                !0, // id
            ]);
    }
    tp.sync();
}

#[test]
fn server_destroyed() {
    let tp = test_proxy();
    tp.client.state.destroy();
    assert!(tp.client.display.new_try_send_sync().is_err());
}

#[test]
fn get_handler() {
    struct H;
    impl WlproxyTestHandler for H {}

    let tp = test_proxy();
    tp.client.test.set_handler(H);
    tp.client.test.get_handler_mut::<H>();
    tp.client.test.get_handler_ref::<H>();
    assert!(tp.client.test.try_get_handler_mut::<H>().is_ok());
    assert!(tp.client.test.try_get_handler_ref::<H>().is_ok());
    let handler = tp.client.test.get_handler_ref::<H>();
    assert!(tp.client.test.try_get_handler_ref::<H>().is_ok());
    assert!(tp.client.test.try_get_handler_mut::<H>().is_err());
    drop(handler);
    let _handler = tp.client.test.get_handler_mut::<H>();
    assert!(tp.client.test.try_get_handler_ref::<H>().is_err());
    assert!(tp.client.test.try_get_handler_mut::<H>().is_err());
}

#[test]
fn state() {
    let tp = test_proxy();
    assert!(Rc::ptr_eq(&tp.client.state, tp.client.test.state()));
}

#[test]
fn client() {
    let tp = test_proxy();
    assert!(Rc::ptr_eq(
        &tp.client.proxy_client,
        tp.client.proxy_test.client().as_ref().unwrap()
    ));
}

#[test]
fn version() {
    let tp = test_proxy();
    assert_eq!(tp.client.test.version(), 1);
}

#[test]
fn client_id() {
    let tp = test_proxy();
    assert_eq!(
        tp.client.test.server_id().unwrap(),
        tp.client.proxy_test.client_id().unwrap()
    );
}

#[test]
fn forward() {
    struct Nfh(bool);
    impl WlproxyTestNonForwardHandler for Nfh {
        fn handle_echoed(&mut self, _slf: &Rc<WlproxyTestNonForward>) {
            self.0 = true;
        }
    }

    struct Th1;
    impl WlproxyTestHandler for Th1 {
        fn handle_create_non_forward(
            &mut self,
            _slf: &Rc<WlproxyTest>,
            id: &Rc<WlproxyTestNonForward>,
        ) {
            id.set_forward_to_server(false);
        }
    }

    struct Th2;
    impl WlproxyTestHandler for Th2 {
        fn handle_create_non_forward(
            &mut self,
            _slf: &Rc<WlproxyTest>,
            id: &Rc<WlproxyTestNonForward>,
        ) {
            id.set_forward_to_client(false);
        }
    }

    let tp = test_proxy();

    let non_forward = tp.client.test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(non_forward.get_handler_mut::<Nfh>().0);

    tp.client.proxy_test.set_handler(Th1);

    let non_forward = tp.client.test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(!non_forward.get_handler_mut::<Nfh>().0);

    tp.client.proxy_test.set_handler(Th2);

    let non_forward = tp.client.test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(!non_forward.get_handler_mut::<Nfh>().0);

    tp.client.proxy_test.unset_handler();

    let non_forward = tp.client.test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(non_forward.get_handler_mut::<Nfh>().0);
}

#[test]
fn event_on_zombie() {
    let tp = test_proxy();

    {
        let (server, client) = tp.get_server_sent_object();
        client.send_send_event_x();
        client.send_destroy();

        struct H(bool);
        impl WlproxyTestServerSentHandler for H {
            fn handle_event_x(&mut self, _slf: &Rc<WlproxyTestServerSent>) {
                self.0 = true;
            }
        }

        server.set_handler(H(false));
        client.set_handler(H(false));

        tp.sync();

        assert!(server.get_handler_mut::<H>().0);
        assert!(!client.get_handler_mut::<H>().0);
    }

    {
        let (_, client) = tp.get_server_sent_object();
        client.send_send_event_x();
        client.send_destroy();

        struct H(bool);
        impl WlproxyTestServerSentHandler for H {
            fn handle_event_x(&mut self, _slf: &Rc<WlproxyTestServerSent>) {
                self.0 = true;
            }
        }

        client.set_handler(H(false));
        tp.sync();
        assert!(!client.get_handler_mut::<H>().0);
    }
}

#[test]
fn zombie_id_reuse() {
    let tp = test_proxy();

    let (p1, c1) = tp.get_server_sent_object();
    let p1_id = p1.server_id();

    c1.send_destroy();

    let (p2, c1) = tp.get_server_sent_object();
    let p2_id = p2.server_id();

    assert_eq!(p1_id, p2_id);

    struct H(bool);
    impl WlproxyTestServerSentHandler for H {
        fn handle_event_x(&mut self, _slf: &Rc<WlproxyTestServerSent>) {
            self.0 = true;
        }
    }

    c1.send_send_event_x();
    c1.set_handler(H(false));
    tp.sync();
    assert!(c1.get_handler_mut::<H>().0);
}

#[test]
#[should_panic(expected = "HasServerId(4278190080)")]
fn reuse_server_sent() {
    let tp = test_proxy();
    let (_, c1) = tp.get_server_sent_object();
    c1.send_destroy();
    tp.client.test.try_send_create_server_sent(&c1).unwrap();
}

#[test]
fn reuse_normal() {
    let tp = test_proxy();
    let c1 = tp.client.test.new_try_send_create_server_sent().unwrap();
    c1.try_send_destroy().unwrap();
    tp.sync();
    tp.client.test.try_send_create_server_sent(&c1).unwrap();
    tp.sync();
}
