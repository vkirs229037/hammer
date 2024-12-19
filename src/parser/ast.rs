use std::collections::HashMap;

use crate::parser::tokens::*;
use crate::parser::errors::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Box<Self>>),
    Expr(Box<Expr>),
    Decl(Variable, Box<Expr>)
}

#[derive(Clone, Debug)]
pub enum Expr {
    Func(Token, Box<Self>),
    Literal(Token), 
    Grouping(Box<Self>),
    Binary(Box<Self>, Token, Box<Self>),
    Unary(Token, Box<Self>),
    Variable(Token),
    None,
}

// Пока что поддерживаются только сразу
// инициализируемые переменные
// Конечно, в жизни никогда так не бывает,
// поэтому в будущем нужно будет это учесть
// TODO
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    name: String,
    initialized: bool
}

pub struct AstBuilder {
    tokens: Vec<Token>,
    cursor: usize,
    pub tree: Vec<Stmt>,
    pub variables: Vec<Variable>
}

pub struct Ast {
    pub tree: Vec<Stmt>,
    pub variables: Vec<Variable>,
}

impl AstBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            tree: vec![],
            variables: vec![]
        }
    }

    pub fn ast(self) -> Ast {
        Ast {
            tree: self.tree, 
            variables: self.variables
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        loop { 
            let stmt;
            match self.peek()?.ttype {
                TokenType::Eof => break,
                TokenType::Keyword(Kw::Let) => {
                    self.consume()?;
                    stmt = self.decl()?;
                    self.tree.push(stmt);
                }
                _ => {
                    stmt = Stmt::Expr(Box::new(self.expr()?));
                    self.tree.push(stmt);
                }
            }
            if !self.match_ttype(&[TokenType::Semicolon])? {
                return Err(ParseError::ExpectedSemi(self.prev().loc.clone()))
            }
        }
        Ok(())
    }

    fn decl(&mut self) -> Result<Stmt, ParseError> {
        let name;
        // См. определение структуры Variable
        let initialized = true;
        let token = &self.consume()?.clone();
        match &token.ttype {
            TokenType::Ident(id) => name = id,
            _ => return Err(ParseError::ExpectedIdent(self.prev().loc.clone()))
        }
        if !self.match_ttype(&[TokenType::Assign])? {
            return Err(ParseError::ExpectedAssign(self.prev().loc.clone()))
        }
        let expr = self.expr()?;
        let var = Variable { name: name.to_string(), initialized };
        self.variables.push(var.clone());
        Ok(Stmt::Decl(var, Box::new(expr)))
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        match &self.peek()?.ttype {
            TokenType::Builtin(_) => {
                let func = self.consume()?.clone();
                if !self.match_ttype(&[TokenType::ParenLeft])? {
                    return Err(ParseError::ExpectedParen(self.peek()?.loc.clone()))
                }
                let expr = self.expr()?;
                if !self.match_ttype(&[TokenType::ParenRight])? {
                    return Err(ParseError::ExpectedParen(self.peek()?.loc.clone()))
                }
                Ok(Expr::Func(func, Box::new(expr)))
            },
            _ => self.term(),
        }
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        
        while self.match_ttype(&[TokenType::OpPlus, TokenType::OpMinus])? {
            let op = self.prev().clone();
            let right = self.factor()?;
            
            expr = Expr::Binary(Box::new(expr), op.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_ttype(&[TokenType::OpStar, TokenType::OpSlash])? {
            let op = self.prev().clone();
            let right = self.unary()?;
            
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_ttype(&[TokenType::OpMinus])? {
            let op = self.prev().clone();
            let expr = self.primary()?;
            
            return Ok(Expr::Unary(op, Box::new(expr)))
        }
        
        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = &self.consume()?.clone();
        
        match &token.ttype {
            TokenType::Eof => Err(ParseError::UnexpectedEof(token.loc.clone())),
            TokenType::NumLit(value) => { 
                Ok(Expr::Literal(token.clone())) 
            },
            TokenType::ParenLeft => {
                let expr = self.expr()?;
                if self.consume()?.ttype != TokenType::ParenRight {
                    return Err(ParseError::UnmatchingBrace(token.loc.clone()));
                }
                Ok(Expr::Grouping(Box::new(expr)))
            },
            TokenType::ParenRight => Err(ParseError::UnmatchingBrace(token.loc.clone())),
            TokenType::Ident(id) => {
                if !self.variables.iter().map(|v| v.name.clone()).any(|name| name == *id) {
                    Err(ParseError::UnknownVariable(token.loc.clone()))
                }
                else {
                    Ok(Expr::Variable(token.clone()))
                }
            },
            _ => Err(ParseError::UnexpectedToken(token.loc.clone()))
        }
    }

    fn match_ttype(&mut self, ttypes: &[TokenType]) -> Result<bool, ParseError> {
        for ttype in ttypes {
            if self.check_ttype(ttype)? {
                self.consume()?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn check_ttype(&self, check_ttype: &TokenType) -> Result<bool, ParseError> {
        if self.eof()? {
            return Ok(false);
        }
        let ttype = &self.peek()?.ttype;
        Ok(ttype == check_ttype)
    }

    fn consume(&mut self) -> Result<&Token, ParseError> {
        if !self.eof()? {
            self.cursor += 1;
        }
        Ok(self.prev())
    }

    fn peek(&self) -> Result<&Token, ParseError> {
        self.tokens.get(self.cursor)
                    .ok_or_else(|| ParseError::UnexpectedEof(self.prev().loc.clone()))
    }

    fn prev(&self) -> &Token {
        self.tokens.get(self.cursor - 1).unwrap()
    }

    fn eof(&self) -> Result<bool, ParseError> {
        let ttype = &self.peek()?.ttype;
        Ok(*ttype == TokenType::Eof)
    }
}