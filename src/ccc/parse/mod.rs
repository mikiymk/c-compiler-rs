pub mod node;
mod parse;
mod token;

use super::error::CompileError;
use node::Node;
use parse::program;
use token::tokenize;

pub fn parse(code: &str) -> Result<Node, CompileError> {
    let mut tokens = tokenize(&code)?;
    program(&mut tokens)
}
