use thiserror::Error;

/// Errors produced by this crate.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) Box<dyn std::error::Error + Send + Sync>);
