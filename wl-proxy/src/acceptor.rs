use {
    crate::xrd::xrd,
    error_reporter::Report,
    std::{
        io,
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::{Errno, c, sockaddr_none_mut},
};

pub struct Acceptor {
    pub id: u64,
    pub socket: OwnedFd,
    pub display: String,
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
    pub fn new(id: u64) -> Result<Rc<Self>, AcceptorError> {
        let xrd = match xrd() {
            Some(d) => d,
            _ => return Err(AcceptorError::XrdNotSet),
        };
        let socket = uapi::socket(
            c::AF_UNIX,
            c::SOCK_STREAM | c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
            0,
        )
        .map_err(|e| AcceptorError::CreateSocket(e.into()))?;
        let socket = socket.into();
        for i in 1..1000 {
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

    pub fn accept(&self) -> Result<Option<OwnedFd>, AcceptorError> {
        let res = uapi::accept4(
            self.socket.as_raw_fd(),
            sockaddr_none_mut(),
            c::SOCK_NONBLOCK | c::SOCK_CLOEXEC,
        );
        match res {
            Ok((s, _)) => Ok(Some(s.into())),
            Err(Errno(c::EAGAIN)) => Ok(None),
            Err(e) => Err(AcceptorError::Accept(e.into())),
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
