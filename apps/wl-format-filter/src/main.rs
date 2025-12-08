#![expect(clippy::from_str_radix_10)]

use {error_reporter::Report, std::io, thiserror::Error, wl_proxy::simple::SimpleProxyError};

mod cli;
mod hf;

#[derive(Debug, Error)]
enum HfError {
    #[error("could not create a simple server")]
    CreateServer(#[source] SimpleProxyError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
    #[error("the server terminated")]
    ServerFailed(#[source] SimpleProxyError),
}

fn main() -> Result<(), Report<HfError>> {
    cli::main().map_err(Report::new)
}
