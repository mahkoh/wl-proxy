use {
    crate::{acceptor::Acceptor, baseline::Baseline, state::State},
    std::os::fd::AsRawFd,
    uapi::{c, c::pollfd, poll},
};

#[test]
fn test() {
    let acceptor = Acceptor::new(1000, true).unwrap();

    assert!(acceptor.accept().unwrap().is_none());

    let poll = || {
        poll(
            &mut [pollfd {
                fd: acceptor.socket().as_raw_fd(),
                events: c::POLLIN,
                revents: 0,
            }],
            0,
        )
        .unwrap()
    };

    assert_eq!(poll(), 0);

    State::builder(Baseline::ALL_OF_THEM)
        .with_server_display_name(acceptor.display())
        .build()
        .unwrap();

    assert_eq!(poll(), 1);
    assert!(acceptor.accept().unwrap().is_some());
    assert_eq!(poll(), 0);
}
