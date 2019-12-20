// Import everything from lexer
use super::*;
use crate::parser::{parse_string, parse_error::ParseErrorKind::LexerError};
use std::io::{Cursor, Write, Seek, SeekFrom};

/* Helpers */
fn assert_lexer_error(input: &str) {
    let e = parse_string(input).expect_err("This test should fail with an error");
    assert_eq!(e.kind, LexerError);
}

fn make_string_reader(s: &str) -> Cursor<Vec<u8>> {
    let mut c = Cursor::new(Vec::new());
    c.write_all(s.as_bytes()).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();
    c
}

fn get_last_id(s: &str) -> Result<String, ParseError> {
    let mut c = make_string_reader(s);
    let mut lexer = Lexer::new(&mut c);

    while {
        lexer.next()?;
        lexer.token != Token::Eof
    } {}
    Ok(lexer.last_id)
}

fn assert_order(s: &str, order: &[Token]) {
    let mut c = make_string_reader(s);
    let mut lexer = Lexer::new(&mut c);
    lexer.next().unwrap();

    for t in order.iter().cloned() {
        assert_eq!(lexer.token, t);
        lexer.next().unwrap();
    }
}

/* Tests */
#[test]
fn id_lexed() {
    assert_order("integer", &[Token::Id]);
}

#[test]
fn multiple_tokens() {
    assert_order("10 integer", &[Token::Number, Token::Id]);
}

#[test]
fn spaces_skipped() {
    let test_string = "   integer   ";
    assert_order(test_string, &[Token::Id]);
    assert_eq!(get_last_id(test_string).unwrap(), "integer");
}

#[test]
fn all_whitespace_skipped() {
    let test_string = "\t\rinteger\nchar   ";
    assert_order(test_string, &[Token::Id, Token::Id]);
    assert_eq!(get_last_id(test_string).unwrap(), "char");
}

#[test]
fn eof_token_added() {
    assert_order("x integer", &[Token::Id, Token::Id, Token::Eof]);
}

#[test]
fn stray_chars_fail() {
    assert_lexer_error("var x : * integer");
}

#[test]
fn multiple_ranges_lexed() {
    assert_order(
        "[1..10,1..11]",
        &[
            Token::OpenSBracket,
            Token::Number,
            Token::DoubleDot,
            Token::Number,
            Token::Comma,
            Token::Number,
            Token::DoubleDot,
            Token::Number,
            Token::CloseSBracket
        ]
    );
}

#[test]
fn full_sample_lexed() {
    assert_order(
        "var x: array [1..10] of integer;",
        &[
            Token::Var,
            Token::Id,
            Token::Colon,
            Token::Array,
            Token::OpenSBracket,
            Token::Number,
            Token::DoubleDot,
            Token::Number,
            Token::CloseSBracket,
            Token::Of,
            Token::Id,
            Token::Semicolon
        ]
    );
}