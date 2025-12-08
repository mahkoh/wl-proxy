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
    tp.client_test.send_send_object();
    struct H(Option<Rc<WlproxyTestServerSent>>);
    impl WlproxyTestHandler for H {
        fn handle_sent_object(&mut self, _slf: &Rc<WlproxyTest>, echo: &Rc<WlproxyTestServerSent>) {
            self.0 = Some(echo.clone());
        }
    }
    tp.client_test.set_handler(H(None));
    tp.sync();
    let sent = tp.client_test.get_handler_mut::<H>().0.take().unwrap();
    struct S(bool);
    impl WlproxyTestServerSentHandler for S {
        fn handle_destroyed(&mut self, _slf: &Rc<WlproxyTestServerSent>) {
            self.0 = true;
        }
    }
    sent.set_handler(S(false));
    sent.send_send_destroy();
    tp.sync();
    assert!(sent.get_handler_mut::<S>().0);
}

#[test]
#[should_panic(expected = "wl_display")]
fn wrong_downcast() {
    let tp = test_proxy();
    (tp.client_display.clone() as Rc<dyn Object>).downcast::<WlKeyboard>();
}

#[test]
fn double_send() {
    let tp = test_proxy();
    let sync = tp.client_display.new_send_sync();
    assert!(tp.client_display.try_send_sync(&sync).is_err());
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
    let dummy = tp.client_test.new_send_create_dummy();
    tp.client_state
        .server
        .as_ref()
        .unwrap()
        .idl
        .release(dummy.core().server_obj_id.take().unwrap());
    tp.client_test.send_create_dummy(&dummy);
    tp.await_client_disconnected();
}

#[test]
fn client_object_with_server_id() {
    let tp = test_proxy();
    tp.client_state
        .server
        .as_ref()
        .unwrap()
        .outgoing
        .borrow_mut()
        .formatter()
        .words([1, 0, !0]);
    tp.client_display.new_send_sync();
    tp.await_client_disconnected();
}

#[test]
fn duplicate_generated_client_id() {
    let tp = test_proxy();
    let ss = tp.proxy_test.new_send_sent_object();
    assert!(tp.proxy_test.try_send_sent_object(&ss).is_err());
}

#[test]
fn destroyed_client() {
    let tp = test_proxy();
    tp.proxy_client.disconnect();
    assert!(tp.proxy_test.new_try_send_sent_object().is_err());
}

#[test]
#[should_panic(expected = "NotServerId(50)")]
fn invalid_server_id() {
    let tp = test_proxy();
    tp.proxy_client
        .endpoint
        .outgoing
        .borrow_mut()
        .formatter()
        .words([
            tp.proxy_test.client_id().unwrap(),
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
        tp.proxy_client
            .endpoint
            .outgoing
            .borrow_mut()
            .formatter()
            .words([
                tp.proxy_test.client_id().unwrap(),
                1,  // event sent_object
                !0, // id
            ]);
    }
    tp.sync();
}

#[test]
fn server_destroyed() {
    let tp = test_proxy();
    tp.client_state.destroy();
    assert!(tp.client_display.new_try_send_sync().is_err());
}

#[test]
fn get_handler() {
    struct H;
    impl WlproxyTestHandler for H {}

    let tp = test_proxy();
    tp.client_test.set_handler(H);
    tp.client_test.get_handler_mut::<H>();
    tp.client_test.get_handler_ref::<H>();
    assert!(tp.client_test.try_get_handler_mut::<H>().is_ok());
    assert!(tp.client_test.try_get_handler_ref::<H>().is_ok());
    let handler = tp.client_test.get_handler_ref::<H>();
    assert!(tp.client_test.try_get_handler_ref::<H>().is_ok());
    assert!(tp.client_test.try_get_handler_mut::<H>().is_err());
    drop(handler);
    let _handler = tp.client_test.get_handler_mut::<H>();
    assert!(tp.client_test.try_get_handler_ref::<H>().is_err());
    assert!(tp.client_test.try_get_handler_mut::<H>().is_err());
}

#[test]
fn state() {
    let tp = test_proxy();
    assert!(Rc::ptr_eq(&tp.client_state, tp.client_test.state()));
}

#[test]
fn client() {
    let tp = test_proxy();
    assert!(Rc::ptr_eq(
        &tp.proxy_client,
        tp.proxy_test.client().as_ref().unwrap()
    ));
}

#[test]
fn version() {
    let tp = test_proxy();
    assert_eq!(tp.client_test.version(), 1);
}

#[test]
fn client_id() {
    let tp = test_proxy();
    assert_eq!(
        tp.client_test.server_id().unwrap(),
        tp.proxy_test.client_id().unwrap()
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

    let non_forward = tp.client_test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(non_forward.get_handler_mut::<Nfh>().0);

    tp.proxy_test.set_handler(Th1);

    let non_forward = tp.client_test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(!non_forward.get_handler_mut::<Nfh>().0);

    tp.proxy_test.set_handler(Th2);

    let non_forward = tp.client_test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(!non_forward.get_handler_mut::<Nfh>().0);

    tp.proxy_test.unset_handler();

    let non_forward = tp.client_test.new_send_create_non_forward();
    non_forward.set_handler(Nfh(false));
    non_forward.send_echo();
    tp.sync();
    assert!(non_forward.get_handler_mut::<Nfh>().0);
}
