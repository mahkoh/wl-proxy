#![expect(
    clippy::single_char_add_str,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil
)]

pub mod acceptor;
pub mod client;
pub mod endpoint;
pub mod fixed;
mod protocol_helpers;
#[rustfmt::skip]
pub mod protocols;
pub mod global_filter;
pub mod object;
mod poll;
pub mod simple;
pub mod state;
mod trans;
mod utils;
mod wire;
