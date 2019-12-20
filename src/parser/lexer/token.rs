use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Id, Number,
    Var, Array, Of,
    Comma, Dot, Semicolon, Colon,
    OpenSBracket, CloseSBracket,
    Epsilon, Eof
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match *self {
            Token::Id | Token::Number => "<id>",
            Token::Var => "var",
            Token::Array => "array",
            Token::Of => "of",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Semicolon => ";",
            Token::Colon => ":",
            Token::OpenSBracket => "[",
            Token::CloseSBracket => "]",
            Token::Epsilon => "ε",
            Token::Eof => "EOF"
        };
        write!(f, "{}", repr)
    }
}