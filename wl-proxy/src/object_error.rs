use {
    crate::{generated::ProxyInterface, proxy::IdError},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ObjectError {
    #[error("could not generate a client id for argument {0}")]
    GenerateClientId(&'static str, #[source] IdError),
    #[error("could not generate a server id for argument {0}")]
    GenerateServerId(&'static str, #[source] IdError),
    #[error("could not assign client id {0} to argument {1}")]
    SetClientId(u32, &'static str, #[source] IdError),
    #[error("could not assign server id {0} to argument {1}")]
    SetServerId(u32, &'static str, #[source] IdError),
    #[error("client {0} has no object with id {1}")]
    NoClientObject(u64, u32),
    #[error("server has no object with id {0}")]
    NoServerObject(u32),
    #[error("argument {} has type {} but should have type {}", .0, .1.name(), .2.name())]
    WrongObjectType(&'static str, ProxyInterface, ProxyInterface),
    #[error("the requested version {} for interface {} is larger than the max version {}", .1, .0.name(), .0.xml_version())]
    MaxVersion(ProxyInterface, u32),
    #[error("the interface {0} is not supported")]
    UnsupportedInterface(String),
    #[error("the receiver has no server id")]
    ReceiverNoServerId,
    #[error("the receiver has no client")]
    ReceiverNoClient,
    #[error("the argument {0} is not associated with client {1}")]
    ArgNoClientId(&'static str, u64),
    #[error("the argument {0} is not associated with a server object")]
    ArgNoServerId(&'static str),
    #[error("the size of the message is {0} instead of {0}")]
    WrongMessageSize(u32, u32),
    #[error("unknown message id {0}")]
    UnknownMessageId(u32),
    #[error("the file descriptor for argument {0} is missing")]
    MissingFd(&'static str),
    #[error("there are trailing bytes after the message")]
    TrailingBytes,
    #[error("argument {0} is not present in the message")]
    MissingArgument(&'static str),
    #[error("argument {0} is a null string but the argument is not nullable")]
    NullString(&'static str),
    #[error("argument {0} is not valid UTF-8")]
    NonUtf8(&'static str),
    #[error("server sent error {} on proxy {}#{}", .2, .0.name(), .1)]
    ServerError(ProxyInterface, u32, u32, #[source] StringError),
    #[error("the message handler is already borrowed")]
    HandlerBorrowed,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct StringError(pub String);
