use std::fmt;
use crate::parser::tokens::Loc;

pub enum LexError {
    MalformedNumLit(Loc),
    UnknownLexem(Loc),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MalformedNumLit(loc) => write!(f, "[{loc}] неправильный float литерал"),
            Self::UnknownLexem(loc) => write!(f, "[{loc}] неизвестная лексема"),
        }
    }
}