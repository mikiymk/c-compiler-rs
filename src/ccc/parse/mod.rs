pub mod node;
mod parse;
mod token;

use crate::ccc::error::CompileError;
use node::Node;
use parse::program;
use token::tokenize;

pub fn parse(code: &str) -> Result<Node, CompileError> {
    let mut token = tokenize(&code)?;
    program(&mut token)
}
