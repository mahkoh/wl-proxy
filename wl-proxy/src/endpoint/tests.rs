use {
    crate::{
        client::Client,
        object::{Object, ObjectCoreApi, ObjectUtils},
        protocols::wlproxy_test::{
            wlproxy_test::{WlproxyTest, WlproxyTestHandler},
            wlproxy_test_dummy::WlproxyTestDummy,
            wlproxy_test_object_echo::{WlproxyTestObjectEcho, WlproxyTestObjectEchoHandler},
        },
        test_framework::proxy::test_proxy,
    },
    std::rc::Rc,
};

#[test]
fn test() {
    let tp = test_proxy();
    {
        let mut outgoing = tp
            .client_state
            .server
            .as_ref()
            .unwrap()
            .outgoing
            .borrow_mut();
        outgoing.formatter().words([1, !0]);
    }
    tp.client_display.new_send_sync();
    tp.await_client_disconnected();
}

#[test]
fn lookup() {
    let tp = test_proxy();
    let dummy = tp.client_test.new_send_create_dummy();
    let echo = tp.client_test.new_send_echo_object(dummy.clone());

    struct H(Option<Rc<dyn Object>>);
    impl WlproxyTestObjectEchoHandler for H {
        fn handle_object(&mut self, _slf: &Rc<WlproxyTestObjectEcho>, obj: Rc<dyn Object>) {
            self.0 = Some(obj);
        }
    }

    echo.set_handler(H(None));
    tp.sync();
    assert_eq!(
        echo.get_handler_mut::<H>().0.take().unwrap().unique_id(),
        dummy.unique_id(),
    );
}

#[test]
fn dispatch_destroyed() {
    struct H(Rc<Client>, u32);
    impl WlproxyTestHandler for H {
        fn handle_create_dummy(&mut self, _slf: &Rc<WlproxyTest>, _id: &Rc<WlproxyTestDummy>) {
            self.0.disconnect();
            self.1 += 1;
        }
    }

    let tp = test_proxy();
    tp.proxy_test.set_handler(H(tp.proxy_client.clone(), 0));
    tp.client_test.new_send_create_dummy();
    tp.client_test.new_send_create_dummy();

    tp.dispatch_blocking();

    assert_eq!(tp.proxy_test.get_handler_mut::<H>().1, 1);
}

#[test]
fn invalid_message() {
    let tp = test_proxy();
    tp.client_state
        .server
        .as_ref()
        .unwrap()
        .outgoing
        .borrow_mut()
        .formatter()
        .words([1, !0u16 as u32]);
    tp.client_display.new_send_sync();
    tp.await_client_disconnected();
}
