use {
    std::{env::var, os::fd::OwnedFd},
    uapi::c::{self, sockaddr_un},
};

pub mod client;
pub mod fixed;
pub mod free_list;
#[rustfmt::skip]
pub mod generated;
pub mod acceptor;
pub mod endpoint;
pub mod generated_helper;
pub mod object_error;
pub mod proxy;
pub mod state;
pub mod trans;
pub mod wire;
pub mod xrd;

fn connect_upstream() -> OwnedFd {
    let name = format!("/run/user/1000/{}", var("WAYLAND_DISPLAY").unwrap());
    let mut addr = sockaddr_un {
        sun_family: c::AF_UNIX as _,
        sun_path: [0; 108],
    };
    let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
    sun_path[..name.len()].copy_from_slice(name.as_bytes());
    sun_path[name.len()] = 0;
    let socket = uapi::socket(c::AF_UNIX, c::SOCK_STREAM | c::SOCK_NONBLOCK, 0).unwrap();
    uapi::connect(socket.raw(), &addr).unwrap();
    socket.into()
}
