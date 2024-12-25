mod cli;
mod compile;
mod error;
mod parser;
mod utils;
mod vm;

use cli::Cli;
use compile::compiler::Compiler;
use error::HammerError;
use parser::ast::AstBuilder;
use parser::lexer::Lexer;
use vm::vm::VM;

use std::env;
use std::fs;
use std::io::Read;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let cli = match Cli::new(&mut args) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            Cli::usage();
            return ExitCode::FAILURE;
        }
    };
    match cli.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}
