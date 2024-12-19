use std::fmt;
use crate::parser::tokens::*;
use std::io;

pub enum CompileError {
    FileError(String, io::Error),
    ExpectedOp(Loc),
    ConstTableOverflow,
    UninitializedVar(Loc)
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileError(filename, error) => write!(f, "[{filename}] {error}"),
            Self::ExpectedOp(loc) => write!(f, "[{loc}] ожидался знак операции"),
            Self::ConstTableOverflow => write!(f, "переполнение таблицы констант"),
            Self::UninitializedVar(loc) => write!(f, "[{loc}] переменная не инициализирована"),
        }
    }
}

