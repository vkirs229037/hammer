use std::fmt;

pub enum CliError {
    UnknownCommand(String),
    NoCommand,
    NoInputFile,
    IncorrectParam(String, String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownCommand(c) => write!(f, "Ошибка: неизвестная команда {c}"),
            Self::NoCommand => write!(f, "Ошибка: не задана команда"),
            Self::NoInputFile => write!(f, "Ошибка: не задан входной файл"),
            Self::IncorrectParam(c, param) => write!(f, "Ошибка: неверный параметр для команды {c}: {param}"),
        }
    }
}