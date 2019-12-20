use std::fmt;
use std::error;
use std::io::{self, Read};

use super::lexer::{Lexer, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    InputError, LexerError, ParserError
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub msg: String
}

impl ParseError {
    pub fn failed_read(e: io::Error) -> ParseError {
        ParseError {
            kind: ParseErrorKind::InputError,
            msg: format!("Read operation failed: {}", e)
        }
    }
    pub fn unknown_terminal(c: char, pos: u32) -> ParseError {
        ParseError {
            kind: ParseErrorKind::LexerError,
            msg: format!("Unknown terminal {} at position {} of input", c, pos)
        }
    }

    pub fn unexpected_token<T: Read>(lexer: &Lexer<T>, expected: &[Token]) -> ParseError {
        ParseError {
            kind: ParseErrorKind::ParserError,
            msg: format!("Unexpected token {} at position {}. Expected one of: {:?}",
                         lexer.token, lexer.position, expected)
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl error::Error for ParseError {
    // Underlying cause is included into the message.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}