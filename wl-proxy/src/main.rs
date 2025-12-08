use {
    crate::state::State,
    std::{env::var, os::fd::OwnedFd},
    uapi::c::{self, sockaddr_un},
};

mod client;
mod fixed;
mod free_list;
#[rustfmt::skip]
mod generated;
mod acceptor;
mod endpoint;
mod generated_helper;
mod object_error;
mod proxy;
mod state;
mod trans;
mod wire;
mod xrd;

fn main() {
    let state = State::new().unwrap();
    let acceptor = state.create_acceptor().unwrap();
    eprintln!("{}", acceptor.display);
    loop {
        state.dispatch_blocking().unwrap();
    }
}

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
