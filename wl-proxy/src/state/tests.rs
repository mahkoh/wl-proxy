use {
    crate::{
        baseline::Baseline,
        client::Client,
        object::{Object, ObjectCoreApi, ObjectUtils},
        protocols::{
            wayland::{
                wl_callback::WlCallback,
                wl_display::{WlDisplay, WlDisplayHandler},
            },
            wlproxy_test::{
                wlproxy_test::{WlproxyTest, WlproxyTestHandler},
                wlproxy_test_array_echo::WlproxyTestArrayEcho,
                wlproxy_test_hops::{WlproxyTestHops, WlproxyTestHopsHandler},
            },
        },
        state::{State, StateHandler},
        test_framework::proxy::{dispatch_blocking, test_proxy, test_proxy_no_log},
    },
    error_reporter::Report,
    std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
        os::fd::AsRawFd,
        rc::Rc,
    },
    uapi::{c, poll},
};

#[test]
fn destructor() {
    let state = State::builder(Baseline::ALL_OF_THEM)
        .without_server()
        .build()
        .unwrap();
    let destructor = state.create_destructor();
    assert!(Rc::ptr_eq(destructor.state(), &state));
    assert!(state.is_not_destroyed());
    assert!(destructor.enabled());
    destructor.disable();
    assert!(!destructor.enabled());
    destructor.enable();
    assert!(destructor.enabled());
    destructor.disable();
    drop(destructor);
    assert!(state.is_not_destroyed());
    let destructor = state.create_destructor();
    drop(destructor);
    assert!(state.is_destroyed());
}

#[test]
fn remote_destructor() {
    let state = State::builder(Baseline::ALL_OF_THEM)
        .without_server()
        .build()
        .unwrap();
    let destructor = state.create_remote_destructor().unwrap();
    state.dispatch_available().unwrap();
    assert!(state.is_not_destroyed());
    assert!(destructor.enabled());
    destructor.disable();
    assert!(!destructor.enabled());
    destructor.enable();
    assert!(destructor.enabled());
    destructor.disable();
    drop(destructor);
    state.dispatch_available().unwrap();
    assert!(state.is_not_destroyed());
    let destructor = state.create_destructor();
    drop(destructor);
    assert!(state.dispatch_available().unwrap_err().is_destroyed());
    assert!(state.is_destroyed());
}

#[test]
fn destroyed_readable() {
    let state = State::builder(Baseline::ALL_OF_THEM)
        .without_server()
        .build()
        .unwrap();
    state.destroy();
    let mut pollfd = [c::pollfd {
        fd: state.poll_fd().as_raw_fd(),
        events: c::POLLIN,
        revents: 0,
    }];
    poll(&mut pollfd, -1).unwrap();
    assert_eq!(pollfd[0].revents & c::POLLIN, c::POLLIN);
}

#[test]
fn add_client() {
    let tp = test_proxy();
    tp.client.proxy_client.disconnect();
}

#[test]
fn acceptor() {
    struct ServerHandler {
        clients: Rc<RefCell<VecDeque<Rc<Client>>>>,
    }
    impl StateHandler for ServerHandler {
        fn new_client(&mut self, client: &Rc<Client>) {
            self.clients.borrow_mut().push_back(client.clone());
        }
    }

    let state1 = State::builder(Baseline::ALL_OF_THEM)
        .without_server()
        .build()
        .unwrap();
    let _destructor = state1.create_destructor();
    let acceptor = state1.create_acceptor(1000).unwrap();

    let clients = Rc::new(RefCell::new(VecDeque::new()));
    state1.set_handler(ServerHandler {
        clients: clients.clone(),
    });

    let state2 = State::builder(Baseline::ALL_OF_THEM)
        .with_server_display_name(acceptor.display())
        .build()
        .unwrap();
    let _destructor = state2.create_destructor();
    state2.display().new_send_sync();

    loop {
        if clients.borrow_mut().pop_front().is_some() {
            break;
        }
        dispatch_blocking([&state1, &state2]).unwrap();
    }
}

#[test]
fn closed_client() {
    let tp = test_proxy();
    tp.client.display.new_send_sync();
    uapi::shutdown(tp.client.fd.as_raw_fd(), c::SHUT_RD).unwrap();
    tp.await_client_disconnected();
}

#[test]
fn many_events() {
    let tp = test_proxy_no_log();
    tp.client.test.send_send_many_events();
    tp.sync();
}

#[test]
fn count_hops() {
    struct Handler;
    impl WlproxyTestHandler for Handler {
        fn handle_count_hops(&mut self, slf: &Rc<WlproxyTest>, id: &Rc<WlproxyTestHops>) {
            id.set_handler(Handler);
            slf.send_count_hops(id);
        }
    }
    impl WlproxyTestHopsHandler for Handler {
        fn handle_count(&mut self, slf: &Rc<WlproxyTestHops>, count: u32) {
            slf.send_count(count + 1);
        }
    }

    struct ClientHandler;
    impl WlproxyTestHopsHandler for ClientHandler {
        fn handle_count(&mut self, _slf: &Rc<WlproxyTestHops>, count: u32) {
            assert_eq!(count, 2);
        }
    }

    let tp = test_proxy_no_log();
    tp.client.proxy_test.set_handler(Handler);
    let hops = tp.client.test.new_send_count_hops();
    hops.set_handler(ClientHandler);
    tp.sync();
}

#[test]
fn recursive_dispatch() {
    struct H(Rc<State>, bool);
    impl WlDisplayHandler for H {
        fn handle_sync(&mut self, slf: &Rc<WlDisplay>, callback: &Rc<WlCallback>) {
            assert!(self.0.dispatch_available().is_err());
            self.1 = true;
            slf.send_sync(callback);
        }
    }

    let tp = test_proxy();
    tp.client
        .proxy_client
        .display()
        .set_handler(H(tp.proxy_state.clone(), false));
    tp.sync();
    assert!(tp.client.proxy_client.display().get_handler_mut::<H>().1);
}

#[test]
fn display_error() {
    struct H(Rc<WlproxyTest>, Rc<Cell<bool>>);
    impl StateHandler for H {
        fn display_error(
            self: Box<Self>,
            object: Option<&Rc<dyn Object>>,
            server_id: u32,
            error: u32,
            msg: &str,
        ) {
            self.1.set(true);
            assert_eq!(object.unwrap().unique_id(), self.0.unique_id());
            assert_eq!(server_id, self.0.server_id().unwrap());
            assert_eq!(error, 2);
            assert_eq!(msg, "abcd");
        }
    }

    let tp = test_proxy();
    let saw_error = Rc::new(Cell::new(false));
    tp.client
        .state
        .set_handler(H(tp.client.test.clone(), saw_error.clone()));
    tp.client
        .proxy_client
        .display
        .send_error(tp.client.proxy_test.clone(), 2, "abcd");
    assert!(!saw_error.get());
    while !saw_error.get() {
        if let Err(e) = dispatch_blocking([&tp.proxy_state, &tp.client.state]) {
            eprintln!("{}", Report::new(e));
        }
    }
}

#[test]
fn suspend1() {
    let tp = test_proxy();
    let client1 = &tp.client;
    let client2 = &tp.create_client();
    let dispatch = || {
        dispatch_blocking([&tp.proxy_state, &client1.state, &client2.state]).unwrap();
    };
    client1.test.new_send_echo_array(b"a");
    client1.test.new_send_echo_array(b"b");
    client1.test.new_send_echo_array(b"c");
    client1.test.new_send_echo_array(b"d");
    client2.test.new_send_echo_array(b"x");
    client2.test.new_send_echo_array(b"y");
    struct H(Vec<Vec<u8>>, bool);
    impl WlproxyTestHandler for H {
        fn handle_echo_array(
            &mut self,
            slf: &Rc<WlproxyTest>,
            echo: &Rc<WlproxyTestArrayEcho>,
            array: &[u8],
        ) {
            echo.send_array(array);
            self.0.push(array.to_vec());
            if self.1 {
                slf.client().unwrap().set_suspended(true);
            }
        }
    }
    client1.proxy_test.set_handler(H(vec![], true));
    client2.proxy_test.set_handler(H(vec![], false));
    let h1 = || client1.proxy_test.get_handler_mut::<H>();
    let h2 = || client2.proxy_test.get_handler_mut::<H>();
    while !client1.proxy_client.endpoint.suspended.get() || h2().0.len() < 2 {
        dispatch();
    }
    assert_eq!(h1().0, [b"a"]);
    assert_eq!(h2().0, [b"x", b"y"]);
    client2.test.new_send_echo_array(b"z");
    while h2().0.len() < 3 {
        dispatch();
    }
    assert_eq!(h2().0, [b"x", b"y", b"z"]);
    client1.proxy_client.set_suspended(false);
    while h1().0.len() < 2 {
        dispatch();
    }
    assert_eq!(h1().0, [b"a", b"b"]);
    client1.proxy_client.set_suspended(false);
    h1().1 = false;
    while h1().0.len() < 4 {
        dispatch();
    }
    assert_eq!(h1().0, [b"a", b"b", b"c", b"d"]);
}

#[test]
fn suspend2() {
    let tp = test_proxy();
    tp.client.test.new_send_echo_array(b"a");
    tp.client.test.new_send_echo_array(b"b");
    tp.client.test.new_send_echo_array(b"c");
    tp.client.test.new_send_echo_array(b"d");
    struct H(Vec<Vec<u8>>);
    impl WlproxyTestHandler for H {
        fn handle_echo_array(
            &mut self,
            slf: &Rc<WlproxyTest>,
            echo: &Rc<WlproxyTestArrayEcho>,
            array: &[u8],
        ) {
            echo.send_array(array);
            self.0.push(array.to_vec());
            slf.client().unwrap().set_suspended(true);
            slf.client().unwrap().set_suspended(false);
        }
    }
    tp.client.proxy_test.set_handler(H(vec![]));
    let h = || tp.client.proxy_test.get_handler_mut::<H>();
    for i in 1..5 {
        while h().0.len() < i {
            tp.dispatch_blocking();
        }
        assert_eq!(h().0.len(), i);
    }
}

#[test]
fn suspend3() {
    let tp = test_proxy();
    tp.client.test.new_send_echo_array(b"a");
    tp.client.test.new_send_echo_array(b"b");
    tp.client.test.new_send_echo_array(b"c");
    tp.client.test.new_send_echo_array(b"d");
    struct H(Vec<Vec<u8>>);
    impl WlproxyTestHandler for H {
        fn handle_echo_array(
            &mut self,
            slf: &Rc<WlproxyTest>,
            echo: &Rc<WlproxyTestArrayEcho>,
            array: &[u8],
        ) {
            echo.send_array(array);
            self.0.push(array.to_vec());
            if array == b"a" {
                slf.client().unwrap().set_suspended(true);
            }
        }
    }
    tp.client.proxy_test.set_handler(H(vec![]));
    let h = || tp.client.proxy_test.get_handler_mut::<H>();
    while h().0.len() < 1 {
        tp.dispatch_blocking();
    }
    assert_eq!(h().0.len(), 1);
    tp.client.proxy_client.set_suspended(false);
    while h().0.len() < 2 {
        tp.dispatch_blocking();
    }
    assert_eq!(h().0.len(), 4);
}

#[test]
fn suspend4() {
    let tp = test_proxy();
    let ch1 = tp.client.test.new_send_count_hops();
    let ch2 = tp.client.test.new_send_count_hops();
    struct TestHandler(Rc<State>);
    impl WlproxyTestHandler for TestHandler {
        fn handle_count_hops(&mut self, slf: &Rc<WlproxyTest>, id: &Rc<WlproxyTestHops>) {
            id.set_handler(ProxyHopsHandler(self.0.clone()));
            slf.send_count_hops(id);
        }
    }
    struct ProxyHopsHandler(Rc<State>);
    impl WlproxyTestHopsHandler for ProxyHopsHandler {
        fn handle_count(&mut self, slf: &Rc<WlproxyTestHops>, count: u32) {
            self.0.set_suspended(true);
            slf.send_count(count + 1);
        }
    }
    struct ClientHopsHandler(u32);
    impl WlproxyTestHopsHandler for ClientHopsHandler {
        fn handle_count(&mut self, _slf: &Rc<WlproxyTestHops>, count: u32) {
            self.0 = count;
        }
    }
    tp.client
        .proxy_test
        .set_handler(TestHandler(tp.proxy_state.clone()));
    ch1.set_handler(ClientHopsHandler(0));
    ch2.set_handler(ClientHopsHandler(0));
    let h1 = || ch1.get_handler_mut::<ClientHopsHandler>();
    let h2 = || ch2.get_handler_mut::<ClientHopsHandler>();
    while h1().0 == 0 {
        tp.dispatch_blocking();
    }
    assert_eq!(h1().0, 2);
    assert_eq!(h2().0, 0);
    log::info!("unsuspend");
    tp.proxy_state.set_suspended(false);
    while h2().0 == 0 {
        tp.dispatch_blocking();
    }
    assert_eq!(h1().0, 2);
    assert_eq!(h2().0, 2);
}

#[test]
fn suspend5() {
    let tp = test_proxy();
    tp.client.test.new_send_echo_array(b"a");
    tp.client.test.new_send_echo_array(b"b");
    struct H(Vec<Vec<u8>>);
    impl WlproxyTestHandler for H {
        fn handle_echo_array(
            &mut self,
            slf: &Rc<WlproxyTest>,
            echo: &Rc<WlproxyTestArrayEcho>,
            array: &[u8],
        ) {
            echo.send_array(array);
            self.0.push(array.to_vec());
            slf.client().unwrap().set_suspended(true);
            slf.client().unwrap().set_suspended(false);
            slf.client().unwrap().set_suspended(true);
        }
    }
    tp.client.proxy_test.set_handler(H(vec![]));
    let h = || tp.client.proxy_test.get_handler_mut::<H>();
    while h().0.is_empty() {
        tp.dispatch_blocking();
    }
    tp.dispatch_blocking();
    assert_eq!(h().0.len(), 1);
}

#[test]
fn suspend6() {
    let tp = test_proxy();
    struct H(bool);
    impl WlproxyTestHandler for H {
        fn handle_echo_array(
            &mut self,
            _slf: &Rc<WlproxyTest>,
            _echo: &Rc<WlproxyTestArrayEcho>,
            _array: &[u8],
        ) {
            self.0 = true;
        }
    }
    tp.client.proxy_test.set_handler(H(false));
    let h = || tp.client.proxy_test.get_handler_mut::<H>();
    tp.client.proxy_client.set_suspended(true);
    tp.client.test.new_send_echo_array(b"a");
    dispatch_blocking([&tp.client.state]).unwrap();
    assert_eq!(tp.proxy_state.dispatch_available().unwrap(), true);
    assert_eq!(h().0, false);
    assert_eq!(tp.proxy_state.dispatch_available().unwrap(), false);
    assert_eq!(h().0, false);
    assert_eq!(tp.proxy_state.dispatch_available().unwrap(), false);
    assert_eq!(h().0, false);
    tp.client.proxy_client.set_suspended(false);
    assert_eq!(tp.proxy_state.dispatch_available().unwrap(), true);
    assert_eq!(h().0, true);
}

#[test]
fn delete_id_for_unknown_object() {
    let (server, proxy) =
        uapi::socketpair(c::AF_UNIX, c::SOCK_STREAM | c::SOCK_CLOEXEC, 0).unwrap();
    let state = State::builder(Baseline::ALL_OF_THEM)
        .with_server_fd(&Rc::new(proxy.into()))
        .build()
        .unwrap();
    // wl_display.delete_id(1234) for an object the proxy never created.
    let msg = [1u32, (12 << 16) | 1, 1234];
    uapi::write(server.raw(), &msg).unwrap();
    // The first dispatch only registers the socket; the event is read in a later one.
    for _ in 0..5 {
        state.dispatch_available().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(state.is_not_destroyed());
}
