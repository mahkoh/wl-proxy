use {
    crate::utils::xrd::xrd,
    error_reporter::Report,
    std::{
        env::set_var,
        io,
        os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::{Errno, c, sockaddr_none_mut},
};

pub struct Acceptor {
    pub(crate) id: u64,
    pub(crate) socket: OwnedFd,
    pub(crate) display: String,
}

#[derive(Debug, Error)]
pub enum AcceptorError {
    #[error("XDG_RUNTIME_DIR is not set")]
    XrdNotSet,
    #[error("could not create a socket")]
    CreateSocket(#[source] io::Error),
    #[error("XDG_RUNTIME_DIR ({0:?}) is too long to form a unix socket address")]
    XrdTooLong(String),
    #[error("could not open the lock file")]
    OpenLockFile(#[source] io::Error),
    #[error("could not lock the lock file")]
    LockLockFile(#[source] io::Error),
    #[error("could not stat the existing socket")]
    SocketStat(#[source] io::Error),
    #[error("could not bind the socket to an address")]
    BindFailed(#[source] io::Error),
    #[error("all wayland addresses in the range 0..1000 are already in use")]
    AddressesInUse,
    #[error("could not start listening for incoming connections")]
    ListenFailed(#[source] io::Error),
    #[error("could not accept new connection")]
    Accept(#[source] io::Error),
}

impl Acceptor {
    pub fn new(max_tries: u32, non_blocking: bool) -> Result<Rc<Self>, AcceptorError> {
        Self::create(0, max_tries, non_blocking)
    }

    pub(crate) fn create(
        id: u64,
        max_tries: u32,
        non_blocking: bool,
    ) -> Result<Rc<Self>, AcceptorError> {
        let xrd = match xrd() {
            Some(d) => d,
            _ => return Err(AcceptorError::XrdNotSet),
        };
        let mut ty = c::SOCK_STREAM | c::SOCK_CLOEXEC;
        if non_blocking {
            ty |= c::SOCK_NONBLOCK;
        }
        let socket =
            uapi::socket(c::AF_UNIX, ty, 0).map_err(|e| AcceptorError::CreateSocket(e.into()))?;
        let socket = socket.into();
        for i in 1..max_tries {
            if let Err(e) = bind_socket(&socket, &xrd, i) {
                log::debug!("Cannot use the wayland-{} socket: {}", i, Report::new(e));
                continue;
            }
            if let Err(e) = uapi::listen(socket.as_raw_fd(), 4096) {
                return Err(AcceptorError::ListenFailed(e.into()));
            }
            return Ok(Rc::new(Acceptor {
                id,
                socket,
                display: format!("wayland-{i}"),
            }));
        }
        Err(AcceptorError::AddressesInUse)
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn socket(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }

    pub unsafe fn setenv(&self) {
        unsafe {
            set_var("WAYLAND_DISPLAY", &self.display);
        }
    }

    pub fn accept(&self) -> Result<Option<OwnedFd>, AcceptorError> {
        loop {
            let res = uapi::accept4(
                self.socket.as_raw_fd(),
                sockaddr_none_mut(),
                c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
            );
            match res {
                Ok((s, _)) => return Ok(Some(s.into())),
                Err(Errno(c::EAGAIN)) => return Ok(None),
                Err(Errno(c::EINTR)) => {}
                Err(e) => return Err(AcceptorError::Accept(e.into())),
            }
        }
    }
}

fn bind_socket(socket: &OwnedFd, xrd: &str, id: u32) -> Result<(), AcceptorError> {
    let mut addr: c::sockaddr_un = uapi::pod_zeroed();
    addr.sun_family = c::AF_UNIX as _;
    let name = format!("wayland-{}", id);
    let path = format!("{}/{}", xrd, name);
    let lock_path = format!("{}.lock", path);
    if path.len() + 1 > addr.sun_path.len() {
        return Err(AcceptorError::XrdTooLong(xrd.to_string()));
    }
    let lock_fd = match uapi::open(&*lock_path, c::O_CREAT | c::O_CLOEXEC | c::O_RDWR, 0o644) {
        Ok(l) => l,
        Err(e) => return Err(AcceptorError::OpenLockFile(e.into())),
    };
    if let Err(e) = uapi::flock(lock_fd.raw(), c::LOCK_EX | c::LOCK_NB) {
        return Err(AcceptorError::LockLockFile(e.into()));
    }
    match uapi::lstat(&*path) {
        Ok(_) => {
            let _ = uapi::unlink(&*path);
        }
        Err(Errno(c::ENOENT)) => {}
        Err(e) => return Err(AcceptorError::SocketStat(e.into())),
    }
    let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
    sun_path[..path.len()].copy_from_slice(path.as_bytes());
    sun_path[path.len()] = 0;
    if let Err(e) = uapi::bind(socket.as_raw_fd(), &addr) {
        return Err(AcceptorError::BindFailed(e.into()));
    }
    Ok(())
}
