mod cli;
mod wtt;

use {
    error_reporter::Report, log::LevelFilter, std::io, thiserror::Error,
    wl_proxy::simple::SimpleServerError,
};

#[derive(Debug, Error)]
enum WttError {
    #[error("could not create a simple server")]
    CreateServer(#[source] SimpleServerError),
    #[error("could not spawn child")]
    SpawnChild(#[source] io::Error),
    #[error("the server terminated")]
    ServerFailed(#[source] SimpleServerError),
}

fn main() -> Result<(), Report<WttError>> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
    cli::main().map_err(Report::new)
}
