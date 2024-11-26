mod vm;
mod parser;
mod utils;
mod compile;
mod cli;
mod error;

use cli::Cli;
use error::HammerError;
use parser::lexer::Lexer;
use parser::ast::{AstBuilder, AstNode};
use compile::compiler::Compiler;
use vm::vm::VM;

use std::fs;
use std::io::Read;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let cli = match Cli::new(&mut args) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            Cli::usage();
            return ExitCode::FAILURE
        }
    };
    match cli.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::FAILURE,
    }
}
