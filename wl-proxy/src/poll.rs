use {
    error_reporter::Report,
    std::{
        io,
        os::fd::{AsRawFd, BorrowedFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::{Errno, c},
};

#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub(crate) enum PollError {
    #[error("could not create epoll fd")]
    Create(#[source] io::Error),
    #[error("could not read epoll events")]
    Read(#[source] io::Error),
    #[error("could not register socket with epoll")]
    Add(#[source] io::Error),
    #[error("could not update epoll interests")]
    Update(#[source] io::Error),
}

pub(crate) const MAX_EVENTS: usize = 16;

pub(crate) const READABLE: u32 = c::EPOLLIN as u32;
pub(crate) const WRITABLE: u32 = c::EPOLLOUT as u32;
pub(crate) const ERROR: u32 = (c::EPOLLERR | c::EPOLLHUP) as u32;

const ONESHOT: u32 = c::EPOLLONESHOT as u32;

const ALL: u32 = READABLE | WRITABLE | ERROR | ONESHOT;

#[derive(Copy, Clone, Default)]
pub(crate) struct PollEvent {
    pub u64: u64,
    pub events: u32,
}

pub(crate) struct Poller {
    epoll: Rc<OwnedFd>,
}

impl Poller {
    pub(crate) fn new() -> Result<Self, PollError> {
        let epoll =
            uapi::epoll_create1(c::EPOLL_CLOEXEC).map_err(|e| PollError::Create(e.into()))?;
        Ok(Self {
            epoll: Rc::new(epoll.into()),
        })
    }

    pub(crate) fn fd(&self) -> &Rc<OwnedFd> {
        &self.epoll
    }

    pub(crate) fn read_events(
        &self,
        timeout: c::c_int,
        events: &mut [PollEvent; MAX_EVENTS],
    ) -> Result<usize, PollError> {
        loop {
            let mut epe = [c::epoll_event { events: 0, u64: 0 }; MAX_EVENTS];
            let res = uapi::epoll_wait(self.epoll.as_raw_fd(), &mut epe, timeout);
            let n = match res {
                Ok(n) => n,
                Err(Errno(c::EINTR)) => continue,
                Err(e) => return Err(PollError::Read(e.into())),
            };
            for i in 0..n {
                let epe = &epe[i];
                let ev = &mut events[i];
                ev.u64 = epe.u64;
                ev.events = epe.events & ALL;
            }
            return Ok(n);
        }
    }

    pub(crate) fn register(&self, id: u64, fd: BorrowedFd<'_>) -> Result<(), PollError> {
        let event = c::epoll_event {
            events: ONESHOT,
            u64: id,
        };
        uapi::epoll_ctl(
            self.epoll.as_raw_fd(),
            c::EPOLL_CTL_ADD,
            fd.as_raw_fd(),
            Some(&event),
        )
        .map_err(|e| PollError::Add(e.into()))
    }

    pub(crate) fn unregister(&self, fd: BorrowedFd<'_>) {
        let res = uapi::epoll_ctl(
            self.epoll.as_raw_fd(),
            c::EPOLL_CTL_DEL,
            fd.as_raw_fd(),
            None,
        );
        if let Err(e) = res {
            log::warn!(
                "Could not remove a file descriptor from epoll: {}",
                Report::new(io::Error::from(e)),
            );
        }
    }

    pub(crate) fn update_interests(
        &self,
        id: u64,
        fd: BorrowedFd<'_>,
        events: u32,
    ) -> Result<(), PollError> {
        let event = c::epoll_event {
            events: events | ONESHOT,
            u64: id,
        };
        uapi::epoll_ctl(
            self.epoll.as_raw_fd(),
            c::EPOLL_CTL_MOD,
            fd.as_raw_fd(),
            Some(&event),
        )
        .map_err(|e| PollError::Update(e.into()))
    }
}
