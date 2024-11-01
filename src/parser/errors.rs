use std::fmt;
use crate::parser::tokens::Loc;

pub enum LexError {
    MalformedNumLit(Loc),
    UnexpectedToken(Loc),
    UnmatchedParens(Loc),
    UnexpectedEOF(Loc),
    UnknownLexem(Loc),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MalformedNumLit(loc) => write!(f, "[{loc}] неправильный float литерал"),
            Self::UnexpectedToken(loc) => write!(f, "[{loc}] неожиданное появление"),
            Self::UnmatchedParens(loc) => write!(f, "[{loc}] скобки были неверно расставлены"),
            Self::UnexpectedEOF(loc) => write!(f, "[{loc}] неожиданный конец файла"),
            Self::UnknownLexem(loc) => write!(f, "[{loc}] неизвестная лексема"),
        }
    }
}