use {
    crate::poll::{MAX_EVENTS, PollEvent, Poller, READABLE},
    std::{
        array,
        collections::HashSet,
        io::{Write, pipe},
        os::fd::{AsFd, OwnedFd},
    },
};

#[test]
fn test() {
    let epoll = Poller::new().unwrap();
    let (r, mut w) = pipe().unwrap();
    let r: OwnedFd = r.into();
    epoll.register(1, r.as_fd()).unwrap();
    epoll.update_interests(1, r.as_fd(), READABLE).unwrap();
    let mut events = [PollEvent::default(); MAX_EVENTS];
    let n = epoll.read_events(0, &mut events).unwrap();
    assert_eq!(n, 0);
    w.write_all(&[0]).unwrap();
    let n = epoll.read_events(0, &mut events).unwrap();
    assert_eq!(n, 1);
    assert_eq!(events[0].u64, 1);
    let n = epoll.read_events(0, &mut events).unwrap();
    assert_eq!(n, 0);
    epoll.update_interests(1, r.as_fd(), READABLE).unwrap();
    let n = epoll.read_events(0, &mut events).unwrap();
    assert_eq!(n, 1);
    assert_eq!(events[0].u64, 1);
    epoll.update_interests(1, r.as_fd(), READABLE).unwrap();
    epoll.unregister(r.as_fd());
    let n = epoll.read_events(0, &mut events).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn many() {
    let mut pipes = array::from_fn::<_, { MAX_EVENTS * 2 }, _>(|_| pipe().unwrap());
    for (_, w) in pipes.iter_mut() {
        w.write_all(&[0]).unwrap();
    }
    let epoll = Poller::new().unwrap();
    for (id, (r, _)) in pipes.iter().enumerate() {
        epoll.register(id as u64, r.as_fd()).unwrap();
        epoll
            .update_interests(id as u64, r.as_fd(), READABLE)
            .unwrap();
    }
    let mut seen_ids = HashSet::new();
    let mut events = [PollEvent::default(); MAX_EVENTS];
    loop {
        let n = epoll.read_events(0, &mut events).unwrap();
        if n == 0 {
            break;
        }
        for e in &events[..n] {
            assert_eq!(seen_ids.insert(e.u64), true);
        }
    }
    assert_eq!(seen_ids.len(), pipes.len());
    for i in 0..pipes.len() {
        assert!(seen_ids.contains(&(i as u64)));
    }
}
