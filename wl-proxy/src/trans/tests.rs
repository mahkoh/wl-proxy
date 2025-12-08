use {
    crate::{
        object::ObjectUtils,
        protocols::wlproxy_test::{
            wlproxy_test_array_echo::{WlproxyTestArrayEcho, WlproxyTestArrayEchoHandler},
            wlproxy_test_fd_echo::{WlproxyTestFdEcho, WlproxyTestFdEchoHandler},
        },
        test_framework::proxy::{test_proxy, test_proxy_no_log},
        trans::{HEADER_SIZE, MAX_MESSAGE_SIZE},
    },
    std::{
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
};

fn weird_message_size(size: u16) {
    let tp = test_proxy();
    {
        let mut outgoing = tp
            .client_state
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
    tp.client_display.new_send_sync();
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
    tp.client_test.new_send_echo_array(&array);
    tp.client_test.new_send_echo_array(&array);
    tp.client_test.new_send_echo_array(&[0u8; 4]);
    tp.sync();
}

#[test]
fn fd() {
    let tp = test_proxy();
    let pool = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    tp.client_test.send_recv_fd(&pool);
    tp.client_test.send_recv_fd(&pool);
    tp.sync();
}

#[test]
fn many_messages() {
    let tp = test_proxy_no_log();
    for _ in 0..100_000 {
        tp.client_display.new_send_sync();
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
            tp.client_display.new_send_sync();
        }
        tp.client_test.send_recv_fd(&pool);
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
        let ewh = tp.client_test.new_send_echo_array(array);
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
    let echo = tp.client_test.new_send_echo_fd(&fd1, &fd2);
    echo.set_handler(Handler(fd1, fd2, false));
    tp.sync();
    assert!(echo.get_handler_mut::<Handler>().2);
}
