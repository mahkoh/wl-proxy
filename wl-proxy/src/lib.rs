#![expect(
    clippy::single_char_add_str,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil
)]

pub mod acceptor;
pub mod client;
pub mod endpoint;
pub mod fixed;
pub mod object_error;
mod protocol_helpers;
#[rustfmt::skip]
pub mod protocols;
pub mod protocol_filter;
pub mod proxy;
pub mod simple;
pub mod state;
mod trans;
mod utils;
mod wire;
