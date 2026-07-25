use {
    error_reporter::Report,
    std::io,
    thiserror::Error,
    wl_proxy::{
        simple::SimpleProxyError,
        state::{StateError, get_wayland_socket},
    },
};

mod cli;
mod hf;
mod libdrm;

#[derive(Debug, Error)]
enum HfError {
    #[error("could not extract WAYLAND_SOCKET")]
    WaylandSocket(#[source] StateError),
    #[error("could not create a simple server")]
    CreateServer(#[source] SimpleProxyError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
    #[error("the server terminated")]
    ServerFailed(#[source] SimpleProxyError),
}

fn main() -> Result<(), Report<HfError>> {
    let wayland_socket = unsafe {
        // SAFETY: only reader of WAYLAND_SOCKET, child processes all remove/replace it
        get_wayland_socket().map_err(HfError::WaylandSocket)?
    };
    cli::main(wayland_socket).map_err(Report::new)
}
