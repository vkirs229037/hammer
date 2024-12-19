use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Ident(String),
    Builtin(BIn),
    NumLit(f64),
    Assign,
    OpPlus,
    OpMinus,
    OpStar,
    OpSlash,
    ParenLeft,
    ParenRight,
    Semicolon,
    Eof,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BIn {
    Println,
    Abs,
}

use TokenType::*;

#[derive(Clone, Debug)]
pub struct Loc {
    file: String,
    line: usize,
    col: usize,
}

impl Loc {
    pub fn new(file: String, line: usize, col: usize) -> Self {
        Self {
            file, 
            line: line+1, 
            col: col+1
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = &self.file;
        let line = self.line;
        let col = self.col;
        write!(f, "{line}, {col} in {file}")
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub loc: Loc,
}

impl Token {
    pub fn new(ttype: TokenType, loc: Loc) -> Self {
        Self {
            ttype, loc
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ttype = &self.ttype;
        let loc = &self.loc;
        match ttype {
            Ident(id) => write!(f, "[{loc}] {id}"),
            NumLit(lit) => write!(f, "[{loc}] {lit}"),
            Builtin(b) => write!(f, "[{loc}] {b}"),
            Assign => write!(f, "[{loc}] ="),
            OpPlus => write!(f, "[{loc}] +"),
            OpMinus => write!(f, "[{loc}] -"),
            OpStar => write!(f, "[{loc}] *"),
            OpSlash => write!(f, "[{loc}] /"),
            ParenLeft => write!(f, "[{loc}] ("),
            ParenRight => write!(f, "[{loc}] )"),
            Semicolon => write!(f, "[{loc}] ;"),
            Eof => write!(f, "[{loc}]"),
        }
    }
}

impl fmt::Display for BIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Abs => write!(f, "builtin abs"),
            Self::Println => write!(f, "builtin println"),
        }
    }
}