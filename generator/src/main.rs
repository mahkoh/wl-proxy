mod ast;
mod builder;
mod error;
mod formatter;
mod parser;
use crate::builder::Builder;
use error::Error;

fn main() {
    Builder::default().build().unwrap();
}
