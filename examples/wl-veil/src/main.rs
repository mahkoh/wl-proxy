use {error_reporter::Report, std::io, thiserror::Error, wl_proxy::acceptor::AcceptorError};

mod cli;
mod veil;

#[derive(Debug, Error)]
enum VeilError {
    #[error("could not create an acceptor")]
    CreateAcceptor(#[source] AcceptorError),
    #[error("could not accept a connection")]
    AccepConnection(#[source] AcceptorError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
}

fn main() -> Result<(), Report<VeilError>> {
    cli::main().map_err(Report::new)
}
