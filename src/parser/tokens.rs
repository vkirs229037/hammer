use std::fmt;

pub enum TokenType {
    Ident(String),
    NumLit(f64),
    OpPlus,
    OpMinus,
    OpStar,
    OpSlash,
    ParenLeft,
    ParenRight,
    Semicolon,
    Eof,
}

use TokenType::*;

pub struct Loc {
    file: String,
    line: usize,
    col: usize,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = &self.file;
        let line = self.line;
        let col = self.col;
        write!(f, "{line}, {col} in {file}")
    }
}

pub struct Token {
    ttype: TokenType,
    loc: Loc,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ttype = &self.ttype;
        let loc = &self.loc;
        match ttype {
            Ident(id) => write!(f, "[{loc}] {id}"),
            NumLit(lit) => write!(f, "[{loc}] {lit}"),
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