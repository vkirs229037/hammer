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

pub enum ParseError {
    UnexpectedEof(Loc),
    UnmatchingBrace(Loc),
    UnexpectedToken(Loc),
    ExpectedSemi(Loc),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedEof(loc) => write!(f, "[{loc}] неожиданный конец файла"),
            Self::UnmatchingBrace(loc) => write!(f, "[{loc}] неверная скобочная последовательность"),
            Self::UnexpectedToken(loc) => write!(f, "[{loc}] неожиданный токен"),
            Self::ExpectedSemi(loc) => write!(f, "[{loc}] ожидалась точка с запятой"),
        }
    }
}