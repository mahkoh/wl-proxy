#![expect(clippy::single_char_add_str, clippy::collapsible_else_if)]

mod ast;
mod collector;
mod formatter;
pub mod generate;
mod parser;
