use crate::parser::tokens::*;
use crate::parser::errors::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Box<Self>>),
    Expr(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Func(Token, Box<Self>),
    Literal(Token), 
    Grouping(Box<Self>),
    Binary(Box<Self>, Token, Box<Self>),
    Unary(Token, Box<Self>),
    None,
}

pub struct AstBuilder {
    tokens: Vec<Token>,
    cursor: usize,
    pub tree: Vec<Stmt>
}

impl AstBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            tree: vec![]
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        loop { 
            let stmt;
            match self.peek()?.ttype {
                TokenType::Eof => break,
                _ => {
                    stmt = Stmt::Expr(Box::new(self.expr()?));
                    self.tree.push(stmt);
                    if !self.match_ttype(&[TokenType::Semicolon])? {
                        return Err(ParseError::ExpectedSemi(self.prev().loc.clone()))
                    }
                }
            }
        }
        Ok(())
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
            TokenType::Ident(id) => todo!("Встречен идентификатор {id} при построении AST"),
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