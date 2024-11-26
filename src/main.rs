mod vm;
mod parser;
mod utils;
mod compile;
mod cli;
mod error;

use parser::lexer::Lexer;
use parser::ast::{AstBuilder, AstNode};
use compile::compiler::Compiler;
use vm::vm::VM;

use std::fs;
use std::io::Read;
use std::env;

fn main() {
    println!("a");
}
