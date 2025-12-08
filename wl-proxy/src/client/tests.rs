use {
    crate::{
        client::ClientHandler,
        object::ObjectCoreApi,
        protocols::ObjectInterface,
        test_framework::proxy::{TestProxy, dispatch_blocking, test_proxy},
    },
    std::{cell::Cell, rc::Rc},
};

#[test]
fn disconnected() {
    let TestProxy {
        _proxy_destructor,
        _client_destructor,
        proxy_state,
        proxy_client,
        proxy_test: _proxy_test,
        client_state,
        client_display,
        client_test,
        client_fd,
    } = test_proxy();

    drop(_client_destructor);
    drop(client_state);
    drop(client_display);
    drop(client_test);
    drop(client_fd);

    struct H(Rc<Cell<bool>>);
    impl ClientHandler for H {
        fn disconnected(self: Box<Self>) {
            self.0.set(true);
        }
    }
    let disconnected = Rc::new(Cell::new(false));
    proxy_client.set_handler(H(disconnected.clone()));
    assert!(!disconnected.get());
    while !disconnected.get() {
        dispatch_blocking([&proxy_state]);
    }

    struct H2(Rc<Cell<bool>>);
    impl ClientHandler for H2 {}
    impl Drop for H2 {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let dropped = Rc::new(Cell::new(false));
    proxy_client.set_handler(H2(dropped.clone()));
    assert!(dropped.get());
}

#[test]
fn unset_handler() {
    struct H2(Rc<Cell<bool>>);
    impl ClientHandler for H2 {}
    impl Drop for H2 {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let dropped = Rc::new(Cell::new(false));
    let tp = test_proxy();
    tp.proxy_client.set_handler(H2(dropped.clone()));

    assert!(!dropped.get());
    tp.proxy_client.unset_handler();
    assert!(dropped.get());
}

#[test]
fn objects() {
    let tp = test_proxy();
    let mut objects = vec![];
    tp.proxy_client.objects(&mut objects);
    assert_eq!(objects.len(), 3);
    let mut has_display = false;
    let mut has_registry = false;
    let mut has_test = false;
    for obj in objects {
        use ObjectInterface::*;
        match obj.interface() {
            WlDisplay => has_display = true,
            WlRegistry => has_registry = true,
            WlproxyTest => has_test = true,
            _ => unreachable!(),
        }
    }
    assert!(has_display);
    assert!(has_registry);
    assert!(has_test);

    let mut objects = vec![];
    tp.proxy_client.disconnect();
    tp.proxy_client.objects(&mut objects);
    assert_eq!(objects.len(), 0);
}
