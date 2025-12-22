#![expect(
    clippy::single_char_add_str,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil
)]

pub mod acceptor;
pub mod client;
mod endpoint;
pub mod fixed;
mod protocol_helpers;
/// Auto-generated wayland protocols.
#[rustfmt::skip]
pub mod protocols;
pub mod baseline;
pub mod global_mapper;
pub mod handler;
pub mod object;
mod poll;
pub mod simple;
pub mod state;
mod trans;
mod utils;
mod wire;
