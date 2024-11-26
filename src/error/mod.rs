use crate::cli::errors::*;
use crate::compile::errors::*;
use crate::parser::errors::*;
use crate::vm::errors::*;

pub enum HammerError {
    Cli(CliError),
    Compile(CompileError),
    Lex(LexError),
    Parse(ParseError),
    Interp(InterpretationError),
    Bytecode(BytecodeError)
}