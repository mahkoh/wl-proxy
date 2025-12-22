#![expect(clippy::identity_op, clippy::collapsible_if)]

mod cli;
mod desktop;
mod icon;
mod wtt;

use {
    error_reporter::Report, log::LevelFilter, std::io, thiserror::Error,
    wl_proxy::simple::SimpleProxyError,
};

#[derive(Debug, Error)]
enum WttError {
    #[error("could not create a simple server")]
    CreateServer(#[source] SimpleProxyError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
    #[error("the server terminated")]
    ServerFailed(#[source] SimpleProxyError),
}

fn main() -> Result<(), Report<WttError>> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
    cli::main().map_err(Report::new)
}
