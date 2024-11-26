use std::fmt;

pub enum InterpretationError {
    OpcodeError,
    UnexpectedEndError,
    BadConstsIndexError,
    EmptyStackError,
    ZeroDivisionError,
}

impl fmt::Display for InterpretationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretationError::OpcodeError => write!(f, "неизвестный код комманды"),
            InterpretationError::UnexpectedEndError => write!(f, "неожиданный конец программы"),
            InterpretationError::BadConstsIndexError => write!(f, "индекс таблицы констант вышел за границы"),
            InterpretationError::EmptyStackError => write!(f, "стек оказался пустым"),
            InterpretationError::ZeroDivisionError => write!(f, "деление на 0"),
        }
    }
}

pub enum BytecodeError {
    UnexpectedEof
}

impl fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BytecodeError::UnexpectedEof => write!(f, "неожиданный конец файла при чтении константы"),
        }
    }
}