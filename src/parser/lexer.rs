use std::{iter::{self, Peekable}, str::Chars};

use crate::parser::tokens::*;
use crate::parser::errors::*;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    file: String,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(file: String, source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            file,
            line: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Result<(), LexError> {
        let source = self.source.clone();
        let mut source_iter = source.chars().peekable();
        let mut buf = String::new();
        while let Some(c) = source_iter.next() {
            match c {
                c if c == '\n' => { 
                    self.line += 1;
                    self.col = 0;
                }
                c if c.is_whitespace() => self.col += 1,
                c if c.is_alphabetic() => {
                    buf = Self::collect_token(c, &mut source_iter, |c| c.is_alphanumeric());
                    self.parse_ident(&buf)?;
                    self.col += buf.len();
                },
                c if c.is_digit(10) => {
                    buf = Self::collect_token(c, &mut source_iter, |c| c.is_digit(10) || c == '.' || c == '-');
                    self.parse_numlit(&buf)?;
                    self.col += buf.len();
                },
                '+' => { self.push_token(TokenType::OpPlus); self.col += 1; }
                '-' => { self.push_token(TokenType::OpMinus); self.col += 1; }
                '*' => { self.push_token(TokenType::OpStar); self.col += 1; }
                '/' => { self.push_token(TokenType::OpSlash); self.col += 1; }
                '(' => { self.push_token(TokenType::ParenLeft); self.col += 1; }
                ')' => { self.push_token(TokenType::ParenRight); self.col += 1; }
                ';' => { self.push_token(TokenType::Semicolon); self.col += 1; }
                _ => return Err(LexError::UnknownLexem(Loc::new(self.file.clone(), self.line, self.col))),
            }
        }
        self.push_token(TokenType::Eof);
        Ok(())
    }

    fn collect_token(c: char, iterator: &mut Peekable<Chars<'_>>, func: impl Fn(char) -> bool) -> String {
        iter::once(c).chain(iter::from_fn(|| iterator.by_ref().next_if(|x| func(*x)))).collect()
    }

    fn parse_ident(&mut self, buf: &String) -> Result<Token, LexError> {
        match buf.as_str() {
            _ => todo!("Идентификаторы пока что не поддерживаются")
        }
    }

    fn parse_numlit(&mut self, buf: &String) -> Result<(), LexError> {
        let loc = Loc::new(self.file.clone(), self.line, self.col);
        let value = buf.parse().map_err(|_| LexError::MalformedNumLit(loc.clone()))?;
        let token = Token::new(TokenType::NumLit(value), loc);
        self.tokens.push(token);
        Ok(())
    }

    fn push_token(&mut self, ttype: TokenType) {
        let token = Token::new(ttype, Loc::new(self.file.clone(), self.line, self.col));
        self.tokens.push(token);
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 
}