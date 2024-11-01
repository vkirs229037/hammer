use std::fmt;
use crate::parser::tokens::Token;

pub enum LexError {
    MalformedNumLit(Token),
    UnexpectedToken(Token),
    UnmatchedParens(Token),
    UnexpectedEOF(Token),
    UnknownLexem(Token),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MalformedNumLit(t) => write!(f, "неправильный float литерал {t}"),
            Self::UnexpectedToken(t) => write!(f, "неожиданное появление {t}"),
            Self::UnmatchedParens(t) => write!(f, "скобки были неверно расставлены {t}"),
            Self::UnexpectedEOF(t) => write!(f, "неожиданный конец файла {t}"),
            Self::UnknownLexem(t) => write!(f, "неизвестная лексема {t}"),
        }
    }
}