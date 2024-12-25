use crate::parser::tokens::Loc;
use std::fmt;

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
    ExpectedParen(Loc),
    ExpectedIdent(Loc),
    ExpectedAssign(Loc),
    UnknownVariable(Loc),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedEof(loc) => write!(f, "[{loc}] неожиданный конец файла"),
            Self::UnmatchingBrace(loc) => {
                write!(f, "[{loc}] неверная скобочная последовательность")
            }
            Self::UnexpectedToken(loc) => write!(f, "[{loc}] неожиданный токен"),
            Self::ExpectedSemi(loc) => write!(f, "[{loc}] ожидалась точка с запятой"),
            Self::ExpectedParen(loc) => write!(f, "[{loc}] ожидалась скобка"),
            Self::ExpectedIdent(loc) => write!(f, "[{loc}] ожидался идентификатор"),
            Self::ExpectedAssign(loc) => write!(f, "[{loc}] ожидался знак присвоения ="),
            Self::UnknownVariable(loc) => write!(f, "[{loc}] неизвестная переменная"),
        }
    }
}
