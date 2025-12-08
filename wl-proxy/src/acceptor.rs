//! Wayland connection acceptor.

use {
    crate::utils::env::{WAYLAND_DISPLAY, XDG_RUNTIME_DIR},
    error_reporter::Report,
    std::{
        env::{set_var, var},
        io,
        os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::{Errno, c, sockaddr_none_mut},
};

#[cfg(test)]
mod tests;

/// A file-system acceptor for wayland connections.
///
/// This represents a socket in the `XDG_RUNTIME_DIR` directory. Its name follows the
/// usual `wayland-N` scheme.
///
/// # Example
///
/// ```
/// # use std::os::fd::{BorrowedFd, OwnedFd};
/// # use wl_proxy::acceptor::Acceptor;
/// # fn handle_wayland_connection(fd: OwnedFd) { }
/// # fn f() {
/// let acceptor = Acceptor::new(1000, false).unwrap();
/// loop {
///     let con = acceptor.accept().unwrap().unwrap();
///     handle_wayland_connection(con);
/// }
/// # }
/// ```
pub struct Acceptor {
    pub(crate) id: u64,
    pub(crate) socket: OwnedFd,
    display: String,
    _lock_fd: OwnedFd,
}

/// An error emitted by an acceptor.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct AcceptorError(#[from] AcceptorErrorType);

#[derive(Debug, Error)]
enum AcceptorErrorType {
    #[error("{} is not set", XDG_RUNTIME_DIR)]
    XrdNotSet,
    #[error("could not create a socket")]
    CreateSocket(#[source] io::Error),
    #[error("{} ({:?}) is too long to form a unix socket address", XDG_RUNTIME_DIR, .0)]
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
    /// Creates a new acceptor.
    ///
    /// This will try to allocate the socket `wayland-N` in the `XDG_RUNTIME_DIR`
    /// directory. The function starts with `N = 1` and then increments `N` until it finds
    /// an unused socket. The maximum value of `N` is determined by the `max_tries`
    /// parameter.
    ///
    /// If `non_blocking` is true, the created socket will be non-blocking, which means
    /// that [`Acceptor::accept`] can return `Ok(None)`. In this case you should use a
    /// mechanism such as epoll to wait for new connections on the socket.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::os::fd::{BorrowedFd, OwnedFd};
    /// # use wl_proxy::acceptor::Acceptor;
    /// # fn handle_wayland_connection(fd: OwnedFd) { }
    /// # fn f() {
    /// let acceptor = Acceptor::new(1000, false).unwrap();
    /// loop {
    ///     let con = acceptor.accept().unwrap().unwrap();
    ///     handle_wayland_connection(con);
    /// }
    /// # }
    /// ```
    pub fn new(max_tries: u32, non_blocking: bool) -> Result<Rc<Self>, AcceptorError> {
        Self::create(0, max_tries, non_blocking)
    }

    pub(crate) fn create(
        id: u64,
        max_tries: u32,
        non_blocking: bool,
    ) -> Result<Rc<Self>, AcceptorError> {
        let xrd = match var(XDG_RUNTIME_DIR) {
            Ok(d) => d,
            _ => return Err(AcceptorErrorType::XrdNotSet.into()),
        };
        let mut ty = c::SOCK_STREAM | c::SOCK_CLOEXEC;
        if non_blocking {
            ty |= c::SOCK_NONBLOCK;
        }
        let socket = uapi::socket(c::AF_UNIX, ty, 0)
            .map_err(|e| AcceptorErrorType::CreateSocket(e.into()))?;
        let socket = socket.into();
        for i in 1..max_tries {
            let lock_fd = match bind_socket(&socket, &xrd, i) {
                Ok(f) => f,
                Err(e) => {
                    log::debug!("Cannot use the wayland-{} socket: {}", i, Report::new(e));
                    continue;
                }
            };
            if let Err(e) = uapi::listen(socket.as_raw_fd(), 1024) {
                return Err(AcceptorErrorType::ListenFailed(e.into()).into());
            }
            return Ok(Rc::new(Acceptor {
                id,
                socket,
                display: format!("wayland-{i}"),
                _lock_fd: lock_fd,
            }));
        }
        Err(AcceptorErrorType::AddressesInUse.into())
    }

    /// Returns the display name of this acceptor, for example, `wayland-1`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_proxy::acceptor::Acceptor;
    /// # fn f() {
    /// let acceptor = Acceptor::new(1000, false).unwrap();
    /// eprintln!("{}", acceptor.display());
    /// # }
    /// ```
    pub fn display(&self) -> &str {
        &self.display
    }

    /// Returns the socket file descriptor of this acceptor.
    ///
    /// This can be used to asynchronously wait for new connections. The returned file
    /// descriptor should not be used to modify the file description. Otherwise, the behavior is
    /// unspecified.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::os::fd::{BorrowedFd, OwnedFd};
    /// # use wl_proxy::acceptor::Acceptor;
    /// # fn wait_for_descriptor_to_become_readable(fd: BorrowedFd<'_>) { }
    /// # fn handle_wayland_connection(fd: OwnedFd) { }
    /// # fn f() {
    /// let acceptor = Acceptor::new(1000, true).unwrap();
    /// loop {
    ///     wait_for_descriptor_to_become_readable(acceptor.socket());
    ///     let con = acceptor.accept().unwrap().unwrap();
    ///     handle_wayland_connection(con);
    /// }
    /// # }
    /// ```
    pub fn socket(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }

    /// Sets the `WAYLAND_DISPLAY` environment variable to the display of this acceptor.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it calls [`set_var`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::os::fd::{BorrowedFd, OwnedFd};
    /// # use wl_proxy::acceptor::Acceptor;
    /// # fn f() {
    /// let acceptor = Acceptor::new(1000, false).unwrap();
    /// unsafe {
    ///     acceptor.setenv();
    /// }
    /// # }
    /// ```
    pub unsafe fn setenv(&self) {
        // SAFETY: The requirement is forwarded to the caller.
        unsafe {
            set_var(WAYLAND_DISPLAY, &self.display);
        }
    }

    /// Accepts a new connection.
    ///
    /// This can return `None` if and only if this acceptor is non-blocking and there is
    /// currently no client trying to connect to this acceptor.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::os::fd::{BorrowedFd, OwnedFd};
    /// # use wl_proxy::acceptor::Acceptor;
    /// # fn handle_wayland_connection(fd: OwnedFd) { }
    /// # fn f() {
    /// let acceptor = Acceptor::new(1000, false).unwrap();
    /// loop {
    ///     let con = acceptor.accept().unwrap().unwrap();
    ///     handle_wayland_connection(con);
    /// }
    /// # }
    /// ```
    pub fn accept(&self) -> Result<Option<OwnedFd>, AcceptorError> {
        loop {
            let res = uapi::accept4(
                self.socket.as_raw_fd(),
                sockaddr_none_mut(),
                c::SOCK_CLOEXEC,
            );
            match res {
                Ok((s, _)) => return Ok(Some(s.into())),
                Err(Errno(c::EAGAIN)) => return Ok(None),
                Err(Errno(c::EINTR)) => {}
                Err(e) => return Err(AcceptorErrorType::Accept(e.into()).into()),
            }
        }
    }
}

impl AsFd for Acceptor {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.socket()
    }
}

fn bind_socket(socket: &OwnedFd, xrd: &str, id: u32) -> Result<OwnedFd, AcceptorErrorType> {
    let mut addr: c::sockaddr_un = uapi::pod_zeroed();
    addr.sun_family = c::AF_UNIX as _;
    let name = format!("wayland-{}", id);
    let path = format!("{}/{}", xrd, name);
    let lock_path = format!("{}.lock", path);
    if path.len() + 1 > addr.sun_path.len() {
        return Err(AcceptorErrorType::XrdTooLong(xrd.to_string()));
    }
    let lock_fd = match uapi::open(&*lock_path, c::O_CREAT | c::O_CLOEXEC | c::O_RDWR, 0o644) {
        Ok(l) => l,
        Err(e) => return Err(AcceptorErrorType::OpenLockFile(e.into())),
    };
    if let Err(e) = uapi::flock(lock_fd.raw(), c::LOCK_EX | c::LOCK_NB) {
        return Err(AcceptorErrorType::LockLockFile(e.into()));
    }
    match uapi::lstat(&*path) {
        Ok(_) => {
            let _ = uapi::unlink(&*path);
        }
        Err(Errno(c::ENOENT)) => {}
        Err(e) => return Err(AcceptorErrorType::SocketStat(e.into())),
    }
    let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
    sun_path[..path.len()].copy_from_slice(path.as_bytes());
    sun_path[path.len()] = 0;
    if let Err(e) = uapi::bind(socket.as_raw_fd(), &addr) {
        return Err(AcceptorErrorType::BindFailed(e.into()));
    }
    Ok(lock_fd.into())
}
