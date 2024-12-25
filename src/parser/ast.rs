use std::collections::HashMap;

use crate::parser::tokens::*;
use crate::parser::errors::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Box<Self>>),
    Expr(Box<Expr>),
    Decl(Variable, Option<Box<Expr>>),
    Reassign(Variable, Box<Expr>)
}

#[derive(Clone, Debug)]
pub enum Expr {
    Func(Token, Box<Self>),
    Literal(Token), 
    Grouping(Box<Self>),
    Binary(Box<Self>, Token, Box<Self>),
    Unary(Token, Box<Self>),
    Variable(Variable, Loc),
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub name: String,
}

pub struct AstBuilder {
    tokens: Vec<Token>,
    cursor: usize,
    tree: Vec<Stmt>,
    variables: Vec<Variable>
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
            variables: self.variables,
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        loop { 
            let stmt;
            match &self.peek()?.ttype {
                TokenType::Eof => break,
                TokenType::Keyword(Kw::Let) => {
                    self.consume()?;
                    stmt = self.decl()?;
                    self.tree.push(stmt);
                },
                // Здесь вполне возможен вызов определенной функции. 
                // Хоть сейчас и не поддерживается такое, 
                // я хочу сейчас обеспечить поддержку этого
                TokenType::Ident(id) => {
                    if self.peek()?.ttype == TokenType::ParenLeft {
                        stmt = Stmt::Expr(Box::new(self.expr()?));
                        self.tree.push(stmt);
                    }
                    else {
                        stmt = self.reassign()?;
                        self.tree.push(stmt);
                    }
                },
                _ => {
                    stmt = Stmt::Expr(Box::new(self.expr()?));
                    self.tree.push(stmt);
                }
            }
            if !self.match_ttype(&[TokenType::Semicolon])? {
                return Err(ParseError::ExpectedSemi(self.prev().loc.clone()))
            }
            dbg!(&self.tree[self.tree.len() - 1]);
            dbg!("--------------------------------------");
        }
        Ok(())
    }

    fn decl(&mut self) -> Result<Stmt, ParseError> {
        let name;
        let token = &self.consume()?.clone();
        match &token.ttype {
            TokenType::Ident(id) => name = id,
            _ => return Err(ParseError::ExpectedIdent(self.prev().loc.clone()))
        }
        if self.match_ttype(&[TokenType::Assign])? {
            let expr = self.expr()?;
            let var = Variable { name: name.to_string() };
            self.variables.push(var.clone());
            Ok(Stmt::Decl(var, Some(Box::new(expr))))
        }
        else {
            let var = Variable { name: name.to_string() };
            self.variables.push(var.clone());
            Ok(Stmt::Decl(var, None))
        }
    }

    fn reassign(&mut self) -> Result<Stmt, ParseError> {
        let token = &self.consume()?.clone();
        let Token { ttype: TokenType::Ident(varname), loc } = token else {
            panic!("Неожиданная ситуация: должен был быть Ident")
        };
        if !self.match_ttype(&[TokenType::Assign])? {
            return Err(ParseError::ExpectedAssign(self.prev().loc.clone()))
        }
        let var;
        let found_var = self.variables.iter().find(|var| var.name == *varname);
        match found_var {
            None => return Err(ParseError::UnknownVariable(loc.clone())),
            Some(v) => var = v.clone(),
        }
        let expr = self.expr()?;
        Ok(Stmt::Reassign(var, Box::new(expr)))
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
                let found_var = self.variables.iter().find(|var| var.name == *id);
                match found_var {
                    None => Err(ParseError::UnknownVariable(token.loc.clone())),
                    Some(var) => Ok(Expr::Variable(var.clone(), token.loc.clone()))
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