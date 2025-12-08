use {error_reporter::Report, std::io, thiserror::Error, wl_proxy::simple::SimpleProxyError};

mod cli;
mod veil;

#[derive(Debug, Error)]
enum VeilError {
    #[error("could not create a simple server")]
    CreateServer(#[source] SimpleProxyError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
    #[error("the server terminated")]
    ServerFailed(#[source] SimpleProxyError),
}

fn main() -> Result<(), Report<VeilError>> {
    cli::main().map_err(Report::new)
}
