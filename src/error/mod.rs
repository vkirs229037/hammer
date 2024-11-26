use crate::cli::errors::*;
use crate::compile::errors::*;
use crate::parser::errors::*;
use crate::vm::errors::*;

use std::fmt;

pub enum HammerError {
    Cli(CliError),
    Compile(CompileError),
    Lex(LexError),
    Parse(ParseError),
    Interp(InterpretationError),
    Bytecode(BytecodeError)
}

impl fmt::Display for HammerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cli(e) => write!(f, "{e}"),
            Self::Compile(e) => write!(f, "{e}"),
            Self::Lex(e) => write!(f, "{e}"),
            Self::Parse(e) => write!(f, "{e}"),
            Self::Interp(e) => write!(f, "{e}"),
            Self::Bytecode(e) => write!(f, "{e}"),
        }
    }
}