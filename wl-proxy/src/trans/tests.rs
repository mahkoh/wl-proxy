use {
    crate::{
        client::ClientHandler,
        object::ObjectUtils,
        protocols::wlproxy_test::{
            wlproxy_test_array_echo::{WlproxyTestArrayEcho, WlproxyTestArrayEchoHandler},
            wlproxy_test_fd_echo::{WlproxyTestFdEcho, WlproxyTestFdEchoHandler},
        },
        test_framework::proxy::{TestProxy, test_proxy, test_proxy_no_log},
        trans::{HEADER_SIZE, MAX_MESSAGE_SIZE},
    },
    std::{
        cell::Cell,
        mem::MaybeUninit,
        os::fd::{AsRawFd, OwnedFd, RawFd},
        rc::Rc,
        slice, thread,
        time::Duration,
    },
    uapi::{Msghdr, c, sockaddr_none_ref},
};

fn weird_message_size(size: u16) {
    let tp = test_proxy();
    {
        let mut outgoing = tp
            .client
            .state
            .server
            .as_ref()
            .unwrap()
            .outgoing
            .borrow_mut();
        let mut buf = outgoing.stash.pop().unwrap_or_default();
        buf.buffer[0] = 1;
        buf.buffer[1] = (size as u32) << 16;
        buf.valid_from_byte = 0;
        buf.valid_to_byte = 8;
        outgoing.pending.push_back(buf);
    }
    tp.client.display.new_send_sync();
    tp.await_client_disconnected();
}

#[test]
fn large_message() {
    weird_message_size(!0);
}

#[test]
fn small_message() {
    weird_message_size(4);
}

#[test]
fn not_word_size() {
    weird_message_size(9);
}

#[test]
fn edge_header() {
    let tp = test_proxy_no_log();
    tp.sync();
    let array = [0u8; MAX_MESSAGE_SIZE - (HEADER_SIZE + 4 + 4 + 4)];
    tp.client.test.new_send_echo_array(&array);
    tp.client.test.new_send_echo_array(&array);
    tp.client.test.new_send_echo_array(&[0u8; 4]);
    tp.sync();
}

#[test]
fn fd() {
    let tp = test_proxy();
    let pool = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    tp.client.test.send_recv_fd(&pool);
    tp.client.test.send_recv_fd(&pool);
    tp.sync();
}

#[test]
fn many_messages() {
    let tp = test_proxy_no_log();
    for _ in 0..100_000 {
        tp.client.display.new_send_sync();
    }
    tp.sync();
    tp.sync();
}

#[test]
fn many_messages_with_fd() {
    let tp = test_proxy_no_log();
    let pool = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    for _ in 0..1_000 {
        for _ in 0..100 {
            tp.client.display.new_send_sync();
        }
        tp.client.test.send_recv_fd(&pool);
    }
    tp.sync();
    tp.sync();
}

#[test]
fn array() {
    const ARRAYS: &[&[u8]] = &[
        &[],
        &[1],
        &[1, 2],
        &[1, 2, 3],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4, 5],
    ];
    let tp = test_proxy();
    struct Handler(&'static [u8], bool);
    impl WlproxyTestArrayEchoHandler for Handler {
        fn handle_array(&mut self, _slf: &Rc<WlproxyTestArrayEcho>, array: &[u8]) {
            assert_eq!(array, self.0);
            self.1 = true;
        }
    }
    for array in ARRAYS {
        let ewh = tp.client.test.new_send_echo_array(array);
        ewh.set_handler(Handler(array, false));
        tp.sync();
        assert!(ewh.get_handler_mut::<Handler>().1);
    }
}

#[test]
fn echo_fd() {
    let fd1 = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    let fd2 = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    let tp = test_proxy();
    struct Handler(Rc<OwnedFd>, Rc<OwnedFd>, bool);
    impl WlproxyTestFdEchoHandler for Handler {
        fn handle_fd(
            &mut self,
            _slf: &Rc<WlproxyTestFdEcho>,
            fd1: &Rc<OwnedFd>,
            fd2: &Rc<OwnedFd>,
        ) {
            assert_eq!(
                uapi::fstat(self.0.as_raw_fd()).unwrap().st_ino,
                uapi::fstat(fd1.as_raw_fd()).unwrap().st_ino,
            );
            assert_eq!(
                uapi::fstat(self.1.as_raw_fd()).unwrap().st_ino,
                uapi::fstat(fd2.as_raw_fd()).unwrap().st_ino,
            );
            assert_ne!(
                uapi::fstat(fd1.as_raw_fd()).unwrap().st_ino,
                uapi::fstat(fd2.as_raw_fd()).unwrap().st_ino,
            );
            self.2 = true;
        }
    }
    let echo = tp.client.test.new_send_echo_fd(&fd1, &fd2);
    echo.set_handler(Handler(fd1, fd2, false));
    tp.sync();
    assert!(echo.get_handler_mut::<Handler>().2);
}

/// Runs only the proxy (the test client is not dispatched, so nothing it receives matters) and reports whether
/// the proxy disconnected the client.
fn disconnected_by_proxy(tp: &TestProxy) -> bool {
    struct H(Rc<Cell<bool>>);
    impl ClientHandler for H {
        fn disconnected(self: Box<Self>) {
            self.0.set(true);
        }
    }
    let disconnected = Rc::new(Cell::new(false));
    tp.client.proxy_client.set_handler(H(disconnected.clone()));
    for _ in 0..50 {
        tp.proxy_state.dispatch_available().unwrap();
        if disconnected.get() {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

#[test]
fn truncated_control_message() {
    let tp = test_proxy();
    // 60 file descriptors, more than the control buffer holds, with the first 4 bytes of a message: without the
    // check the excess fds would be dropped silently and the proxy would just wait for the rest of the message.
    let fds: Vec<OwnedFd> = (0..60)
        .map(|_| uapi::memfd_create("", c::MFD_CLOEXEC).unwrap().into())
        .collect();
    let raw: Vec<RawFd> = fds.iter().map(|f| f.as_raw_fd()).collect();
    let mut control = vec![MaybeUninit::<u8>::uninit(); uapi::cmsg_space(size_of_val(&raw[..]))];
    let mut hdr: c::cmsghdr = uapi::pod_zeroed();
    hdr.cmsg_level = c::SOL_SOCKET;
    hdr.cmsg_type = c::SCM_RIGHTS;
    uapi::cmsg_write(&mut &mut control[..], hdr, &raw[..]).unwrap();
    let msg = [1u32];
    let iov = uapi::as_bytes(&msg);
    let msghdr = Msghdr {
        iov: slice::from_ref(&iov),
        control: Some(&control[..]),
        name: sockaddr_none_ref(),
    };
    uapi::sendmsg(tp.client.fd.as_raw_fd(), &msghdr, 0).unwrap();
    assert!(disconnected_by_proxy(&tp));
}
