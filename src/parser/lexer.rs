use crate::parser::tokens::*;
use crate::parser::errors::*;

pub struct Lexer {
    source: String,
    cursor: usize,
    tokens: Vec<Token>,
    fail: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            cursor: 0,
            tokens: vec![],
            fail: false
        }
    }

    pub fn lex() -> Result<(), LexError> {
        Ok(())
    }
}