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
    tp.proxy_client.disconnect();
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
    tp.client_display.new_send_sync();
    uapi::shutdown(tp.client_fd.as_raw_fd(), c::SHUT_RD).unwrap();
    tp.await_client_disconnected();
}

#[test]
fn many_events() {
    let tp = test_proxy_no_log();
    tp.client_test.send_send_many_events();
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
    tp.proxy_test.set_handler(Handler);
    let hops = tp.client_test.new_send_count_hops();
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
    tp.proxy_client
        .display()
        .set_handler(H(tp.proxy_state.clone(), false));
    tp.sync();
    assert!(tp.proxy_client.display().get_handler_mut::<H>().1);
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
    tp.client_state
        .set_handler(H(tp.client_test.clone(), saw_error.clone()));
    tp.proxy_client
        .display
        .send_error(tp.proxy_test.clone(), 2, "abcd");
    assert!(!saw_error.get());
    while !saw_error.get() {
        if let Err(e) = dispatch_blocking([&tp.proxy_state, &tp.client_state]) {
            eprintln!("{}", Report::new(e));
        }
    }
}
