// Import everything from parser
use super::*;
use parse_error::ParseErrorKind::ParserError;

/* Helpers */
fn assert_parser_error(input: &str) {
    let e = parse_string(input).expect_err("This test should fail with an error");
    assert_eq!(e.kind, ParserError);
}

fn extract_numbers(n: &Node) -> Result<Vec<String>, ParseError> {
    fn vec_concat<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        a.into_iter().chain(b.into_iter()).collect()
    }
    match n.name.as_str() {
        "S" => extract_numbers(&n.children[5]),
        "D" => Ok(vec_concat(extract_numbers(&n.children[0])?,
                             extract_numbers(&n.children[1])?)),
        "R" => Ok(vec_concat(extract_numbers(&n.children[0])?,
                             extract_numbers(&n.children[3])?)),
        "D\'" => Ok(vec_concat(extract_numbers(&n.children[1])?,
                               extract_numbers(&n.children[2])?)),
        "P" | "D\'\'" => {
            if n.children[0].name != Token::Epsilon.to_string() {
                extract_numbers(&n.children[0])
            } else {
                Ok(Vec::new())
            }
        }
        "N" => Ok(vec![n.children[0].name.clone()]),
        _ => Err(ParseError {kind: ParserError, msg: "Unexpected node name".to_string()})
    }
}

/* Test successful parsing */
#[test]
fn basic_sample() -> Result<(), ParseError> {
    parse_string("var x: array [1..10] of integer;")?;
    Ok(())
}

#[test]
fn multiple_names() -> Result<(), ParseError> {
    parse_string("var x,y,z: array [1..10] of integer;")?;
    Ok(())
}

#[test]
fn multiple_dimensions() -> Result<(), ParseError> {
    parse_string("var x: array [1..100,2..200,3..300] of integer;")?;
    Ok(())
}

/* Test failing parsing */
#[test]
fn no_var() {
    assert_parser_error("x: array [1..10] of integer;");
}

#[test]
fn no_name() {
    assert_parser_error("var : array [1..10] of integer;");
}

#[test]
fn multiple_names_without_comma() {
    assert_parser_error("var x y: array [1..10] of integer;");
}

#[test]
fn no_colon() {
    assert_parser_error("var x array [1..10] of integer;");
}

#[test]
fn no_array() {
    assert_parser_error("var x: [1..10] of integer;");
}

#[test]
fn no_range() {
    assert_parser_error("var x: array of integer;");
}

#[test]
fn no_dots_in_range() {
    assert_parser_error("var x: array [110] of integer;");
}

#[test]
fn too_many_range_dots() {
    assert_parser_error("var x: array [1....10] of integer;");
}

#[test]
fn no_comma_between_ranges() {
    assert_parser_error("var x: array [1..1002..200] of integer;");
}

#[test]
fn multiple_commas_between_ranges() {
    assert_parser_error("var x: array [1..100,,2..200] of integer;");
}

#[test]
fn range_left_bracket_missing() {
    assert_parser_error("var x: array 1..100,2..200] of integer;");
}

#[test]
fn range_right_bracket_missing() {
    assert_parser_error("var x: array [1..100,2..200 of integer;");
}

#[test]
fn no_of() {
    assert_parser_error("var x: array [1..10] integer;");
}

#[test]
fn no_type() {
    assert_parser_error("var x: array [1..10] of ;");
}

#[test]
fn multiple_types() {
    assert_parser_error("var x: array [1..10] of real integer;");
}

#[test]
fn no_semicolon() {
    assert_parser_error("var x: array [1..10] of integer");
}

/* Test correct extraction */
#[test]
fn correct_array_name() {
    let tree = parse_string("var foobar: array [0..5] of integer;").unwrap();
    assert_eq!(tree.children[1].children[0].name, "foobar");
}

#[test]
fn correct_array_type() {
    let tree = parse_string("var x: array [0..5] of char;").unwrap();
    assert_eq!(tree.children[8].children[0].name, "char");
}

#[test]
fn correct_ranges() {
    let tree = parse_string("var x: array [1..101,2..202] of integer;").unwrap();
    let nums = extract_numbers(&tree).unwrap();
    assert_eq!(nums, ["1", "101", "2", "202"]);
}