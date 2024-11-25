use std::fmt;
use crate::parser::tokens::*;
use std::io;

pub enum CompileError {
    FileError(String, io::Error),
    ExpectedOp(Loc)
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileError(filename, error) => write!(f, "[{filename}] ошибка: {error}"),
            Self::ExpectedOp(loc) => write!(f, "[{loc}] ошибка: ожидался знак операции")
        }
    }
}

