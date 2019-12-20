mod lexer;
mod node;
mod parse_error;

use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use lexer::{Lexer, Token};
use node::Node;
use parse_error::ParseError;

pub struct Parser<'a, T: Read> {
    lexer: Lexer<'a, T>
}

pub fn parse_string(s: &str) -> Result<Node, ParseError> {
    let mut c = Cursor::new(Vec::new());
    c.write_all(s.as_bytes()).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();
    Parser::new(Lexer::new(&mut c)).parse()
}

impl<'a, T: Read> Parser<'a, T> {
    pub fn new(lexer: Lexer<T>) -> Parser<T> { Parser { lexer } }

    pub fn parse_file(reader: &mut T) -> Result<Node, ParseError> {
        Parser::new(Lexer::new(reader)).parse()
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        match self.lexer.next() {
            Ok(_) => self.s(),
            Err(e) => Err(e)
        }
    }

    fn skip(&mut self, token: Token) -> Result<Node, ParseError> {
        if self.lexer.token != token && token != Token::Epsilon {
            return Err(ParseError::unexpected_token(&self.lexer, &[token]));
        }

        let res_name =
            if (token == Token::Id || token == Token::Number) && !self.lexer.last_id.is_empty() {
                self.lexer.last_id.clone()
            } else {
                token.to_string()
            };
        
        let res = Node {
            name: res_name,
            children: Vec::new(),
            is_term: true
        };

        if token != Token::Epsilon {
            self.lexer.next()?;
        }
        Ok(res)
    }

    fn s(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Var => Ok(Node::new(
                "S",
                vec![
                    self.skip(Token::Var)?,
                    self.x()?,
                    self.skip(Token::Colon)?,
                    self.skip(Token::Array)?,
                    self.skip(Token::OpenSBracket)?,
                    self.d()?,
                    self.skip(Token::CloseSBracket)?,
                    self.skip(Token::Of)?,
                    self.t()?,
                    self.skip(Token::Semicolon)?
                ])
            ),
            _ => Err(ParseError::unexpected_token(&self.lexer, &[Token::Var]))
        }
    }

    fn t(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("T", vec![self.skip(Token::Id)?]))
    }

    fn d(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("D", vec![self.r()?, self.dss()?]))
    }

    fn ds(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("D\'",
                     vec![self.skip(Token::Comma)?, self.d()?, self.p()?]))
    }

    fn dss(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Comma         => Ok(Node::new("D\'\'", vec![self.ds()?])),
            Token::CloseSBracket => Ok(Node::new("D\'\'",
                                                 vec![self.skip(Token::Epsilon)?])),
            _ => Err(ParseError::unexpected_token(&self.lexer,
                                                  &[Token::Comma, Token::CloseSBracket]))
        }
    }

    fn p(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Comma         => Ok(Node::new("P", vec![self.ds()?])),
            Token::CloseSBracket => Ok(Node::new("P",
                                                 vec![self.skip(Token::Epsilon)?])),
            _ => Err(ParseError::unexpected_token(&self.lexer,
                                                  &[Token::Comma, Token::CloseSBracket]))
        }
    }

    fn r(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Number => Ok(Node::new("R",
                                          vec![self.n()?, self.skip(Token::DoubleDot)?,
                                               self.n()?])),
            Token::Id => Ok(Node::new("T", vec![self.skip(Token::Id)?])),
            _ => Err(ParseError::unexpected_token(&self.lexer, &[Token::Number, Token::Id]))
        }
    }

    fn n(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("N",
                     vec![self.skip(Token::Number)?]))
    }

    fn x(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("X",
                     vec![self.skip(Token::Id)?, self.xss()?]))
    }

    fn xs(&mut self) -> Result<Node, ParseError> {
        Ok(Node::new("X\'",
                     vec![self.skip(Token::Comma)?, self.x()?, self.y()?]))
    }

    fn xss(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Comma => Ok(Node::new("X\'\'", vec![self.xs()?])),
            Token::Colon => Ok(Node::new("X\'\'",
                                         vec![self.skip(Token::Epsilon)?])),
            _ => Err(ParseError::unexpected_token(&self.lexer,
                                              &[Token::Comma, Token::Colon]))
        }
    }

    fn y(&mut self) -> Result<Node, ParseError> {
        match self.lexer.token {
            Token::Comma => Ok(Node::new("Y", vec![self.xs()?])),
            Token::Colon => Ok(Node::new("Y",
                                         vec![self.skip(Token::Epsilon)?])),
            _ => Err(ParseError::unexpected_token(&self.lexer,
                                              &[Token::Comma, Token::Colon]))
        }
    }
}

#[cfg(test)]
mod tests;