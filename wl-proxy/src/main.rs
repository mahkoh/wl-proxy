use arrayvec::ArrayVec;
use std::env::var;
use uapi::c::{POLLIN, POLLOUT, pollfd, sockaddr_un};
use uapi::{OwnedFd, c, sockaddr_none_mut};

mod client;
mod fixed;
mod free_list;
#[rustfmt::skip]
mod generated;
mod generated_helper;
mod object_error;
mod proxy;
mod state;
mod trans;
mod wire;

fn main() {
    let upstream = connect_upstream();
    let name = b"/run/user/1000/wayland-x";
    let _ = uapi::unlink(name.as_slice());
    let mut addr = sockaddr_un {
        sun_family: c::AF_UNIX as _,
        sun_path: [0; 108],
    };
    let sun_path = uapi::as_bytes_mut(&mut addr.sun_path[..]);
    sun_path[..name.len()].copy_from_slice(name);
    sun_path[name.len()] = 0;
    let socket = uapi::socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
    uapi::bind(socket.raw(), &addr).unwrap();
    uapi::listen(socket.raw(), 128).unwrap();
    let (peer, _) = uapi::accept4(socket.raw(), sockaddr_none_mut(), c::SOCK_NONBLOCK).unwrap();
    let want_upstream_write = false;
    let want_upstream_read = false;
    let want_peer_write = false;
    let want_peer_read = true;
    loop {
        let mut polls = ArrayVec::<pollfd, 2>::new();
        if want_upstream_write || want_upstream_read {
            let mut pollfd = pollfd {
                fd: upstream.raw(),
                events: 0,
                revents: 0,
            };
            if want_upstream_write {
                pollfd.events |= POLLOUT;
            }
            if want_upstream_read {
                pollfd.events |= POLLIN;
            }
            polls.push(pollfd);
        }
        if want_peer_write || want_peer_read {
            let mut pollfd = pollfd {
                fd: peer.raw(),
                events: 0,
                revents: 0,
            };
            if want_peer_write {
                pollfd.events |= POLLOUT;
            }
            if want_peer_read {
                pollfd.events |= POLLIN;
            }
            polls.push(pollfd);
        }
        uapi::poll(&mut polls, -1).unwrap();
        for fd in polls {
            if fd.fd == upstream.raw() {
                if fd.revents & POLLIN != 0 {}
                if fd.revents & POLLOUT != 0 {}
            }
            if fd.fd == peer.raw() {
                if fd.revents & POLLIN != 0 {
                    let mut hdr = [0u32; 2];
                    let mut body = [0u32; 1024];
                    let n = uapi::read(peer.raw(), &mut hdr).unwrap();
                    assert_eq!(n.len(), 8);
                    let obj_id = hdr[0];
                    let mut len = hdr[1] >> 16;
                    let req = hdr[1] & 0xffff;
                    dbg!(obj_id, len, req);
                    len -= 8;
                    let body = &mut body[..len as usize / 4];
                    uapi::read(peer.raw(), body).unwrap();
                    dbg!(body);
                }
                if fd.revents & POLLOUT != 0 {}
            }
        }
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
    socket
}
