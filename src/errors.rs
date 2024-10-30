use std::fmt;

pub enum InterpretationError {
    OpcodeError(OpcodeError),
    UnexpectedEndError(UnexpectedEndError),
    BadConstsIndexError(BadConstsIndexError),
    EmptyStackError(EmptyStackError)
}

impl fmt::Display for InterpretationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretationError::OpcodeError(_) => write!(f, "неизвестный код комманды"),
            InterpretationError::UnexpectedEndError(_) => write!(f, "неожиданный конец программы"),
            InterpretationError::BadConstsIndexError(_) => write!(f, "индекс таблицы констант вышел за границы"),
            InterpretationError::EmptyStackError(_) => write!(f, "стек оказался пустым"),
        }
    }
}


pub struct OpcodeError;

impl From<OpcodeError> for InterpretationError {
    fn from(e: OpcodeError) -> Self {
        InterpretationError::OpcodeError(e)
    }
}


pub struct UnexpectedEndError;

impl From<UnexpectedEndError> for InterpretationError {
    fn from(e: UnexpectedEndError) -> Self {
        InterpretationError::UnexpectedEndError(e)
    }
}


pub struct BadConstsIndexError;

impl From<BadConstsIndexError> for InterpretationError {
    fn from(e: BadConstsIndexError) -> Self {
        InterpretationError::BadConstsIndexError(e)
    }
}


pub struct EmptyStackError;

impl From<EmptyStackError> for InterpretationError {
    fn from(e: EmptyStackError) -> Self {
        InterpretationError::EmptyStackError(e)
    }
}