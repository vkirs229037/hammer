use std::{fmt, io::Empty};

pub enum InterpretationError {
    OpcodeError(OpcodeError),
    UnexpectedEndError(UnexpectedEndError),
    BadConstsIndexError(BadConstsIndexError),
    EmptyStackError(EmptyStackError)
}


pub struct OpcodeError;

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неизвестный код комманды")
    }
}

impl From<OpcodeError> for InterpretationError {
    fn from(e: OpcodeError) -> Self {
        InterpretationError::OpcodeError(e)
    }
}


pub struct UnexpectedEndError;

impl fmt::Display for UnexpectedEndError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неожиданный конец программы")
    }
}

impl From<UnexpectedEndError> for InterpretationError {
    fn from(e: UnexpectedEndError) -> Self {
        InterpretationError::UnexpectedEndError(e)
    }
}


pub struct BadConstsIndexError;

impl fmt::Display for BadConstsIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "индекс таблицы констант вышел за границы")
    }
}

impl From<BadConstsIndexError> for InterpretationError {
    fn from(e: BadConstsIndexError) -> Self {
        InterpretationError::BadConstsIndexError(e)
    }
}


pub struct EmptyStackError;

impl fmt::Display for EmptyStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "стек пустой")
    }
}

impl From<EmptyStackError> for InterpretationError {
    fn from(e: EmptyStackError) -> Self {
        InterpretationError::EmptyStackError(e)
    }
}