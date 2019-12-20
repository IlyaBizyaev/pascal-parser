mod token;

use std::io::{self, Read};

pub use token::Token;
use super::parse_error::ParseError;

pub struct Lexer<'a, T: Read> {
    pub token: Token,
    pub position: u32,
    pub last_id: String,
    reader: &'a mut T,
    cur: u8,
}

impl<'a, T: Read> Lexer<'a, T> {
    pub fn new(reader: &'a mut T) -> Lexer<'a, T> {
        Lexer {
            token: Token::Eof,
            position: 1,
            last_id: String::new(),
            reader,
            cur: b' ',
        }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        while self.cur.is_ascii_whitespace() {
            self.read().map_err(ParseError::failed_read)?;
        }

        let mut is_id = false;
        let mut cur_id = String::new();

        while self.cur.is_ascii_alphanumeric() {
            is_id = true;
            cur_id.push(self.cur as char);
            self.read().map_err(ParseError::failed_read)?;
        }

        if is_id {
            for t in [Token::Var, Token::Array, Token::Of].iter().cloned() {
                if cur_id == t.to_string() {
                    self.token = t;
                    return Ok(self.token.clone());
                }
            }
            self.token = match cur_id.parse::<u32>() {
                Ok(_) => Token::Number,
                Err(_) => Token::Id
            };
            self.last_id = cur_id;
            return Ok(self.token.clone());
        }

        if self.cur == b'.' {
            cur_id.push('.');
            self.read().map_err(ParseError::failed_read)?;
            if self.cur == b'.' {
                cur_id.push('.');
                self.read().map_err(ParseError::failed_read)?;
                self.token = Token::DoubleDot;
                return Ok(Token::DoubleDot);
            }
        }

        if self.cur == 0 {
            self.token = Token::Eof;
            Ok(Token::Eof)
        } else {
            let result = match self.cur as char {
                ',' => Ok(Token::Comma),
                ';' => Ok(Token::Semicolon),
                ':' => Ok(Token::Colon),
                '[' => Ok(Token::OpenSBracket),
                ']' => Ok(Token::CloseSBracket),
                other => Err(ParseError::unknown_terminal(other, self.position))
            };
            match result {
                Ok(token) => match self.read() {
                    Ok(_) => {
                        self.token = token;
                        Ok(self.token.clone())
                    },
                    Err(e) => Err(ParseError::failed_read(e))
                },
                Err(e) => Err(e)
            }
        }
    }

    fn read(&mut self) -> Result<u8, io::Error> {
        let mut buffer: [u8; 1] = [0; 1];
        match self.reader.read(&mut buffer) {
            Ok(0) => {
                self.cur = 0;
                Ok(self.cur)
            }
            Ok(_) => {
                self.position += 1;
                self.cur = buffer[0];
                Ok(self.cur)
            },
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests;