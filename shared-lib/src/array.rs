use crate::types::{Array, ValueToken};
use crate::value::parse_value;
use regex::Regex;

enum Mode {
    Scanning,
    Element,
    Delimiter,
    End,
}

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_array(array: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let mut values: Vec<Box<ValueToken>> = Vec::new();

    while let Some(ch) = array.chars().nth(pos) {
        let char = &ch.to_string()[..];

        match mode {
            Mode::Scanning => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '[' {
                    pos += 1;
                    mode = Mode::Element;
                } else {
                    return Err("Expected '['");
                }
            }
            Mode::Element => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == ']' {
                    if !values.is_empty() {
                        return Err("Unexpected ','");
                    }

                    pos += 1;
                    mode = Mode::End;
                } else {
                    let slice: String = array.chars().skip(pos).collect();
                    match parse_value(&slice, r"[ \n\r\t\],]") {
                        Ok(ValueToken::ArrayToken { skip, token }) => {
                            values.push(Box::new(ValueToken::ArrayToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::FalseToken { skip, token }) => {
                            values.push(Box::new(ValueToken::FalseToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::NullToken { skip }) => {
                            values.push(Box::new(ValueToken::NullToken { skip }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::NumberToken { skip, token }) => {
                            values.push(Box::new(ValueToken::NumberToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::ObjectToken { skip, token }) => {
                            values.push(Box::new(ValueToken::ObjectToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::StringToken { skip, token }) => {
                            values.push(Box::new(ValueToken::StringToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(ValueToken::TrueToken { skip, token }) => {
                            values.push(Box::new(ValueToken::TrueToken { skip, token }));
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        _ => {
                            return Err("Unexpected token");
                        }
                    }
                }
            }
            Mode::Delimiter => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == ',' {
                    pos += 1;
                    mode = Mode::Element;
                } else if ch == ']' {
                    pos += 1;
                    mode = Mode::End;
                } else {
                    return Err("Expected ',' or ']'");
                }
            }
            Mode::End => break,
        }
    }

    Ok(ValueToken::ArrayToken {
        skip: pos,
        token: Array { values },
    })
}

#[cfg(test)]
mod tests {
    use crate::json::parse;
    use crate::types::{Json, ValueToken};

    #[test]
    fn array_empty() {
        let input = "[]";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(2, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(2, skip);
                        assert_eq!(0, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_empty_spaces() {
        let input = " [ ] ";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(4, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(3, skip);
                        assert_eq!(0, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_nested_empty() {
        let input = "[[]]";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(4, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(4, skip);
                        assert_eq!(1, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_nested_empty_spaces() {
        let input = " [ [ ] ] ";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(8, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(7, skip);
                        assert_eq!(1, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_mixed_types() {
        let input = "[[], false]";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(11, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(11, skip);
                        assert_eq!(2, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_mixed_types_more() {
        let input = "[[], false, null]";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(17, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(17, skip);
                        assert_eq!(3, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_numbers() {
        let input = "[[], false, null, 1.2e3]";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(24, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(24, skip);
                        assert_eq!(4, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_objects() {
        let input = r#"[[], false, null, 1.2e3, {}]"#;
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(28, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(28, skip);
                        assert_eq!(5, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_strings() {
        let input = r#"[[], false, null, 1.2e3, {}, "Hello, world!"]"#;
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(45, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::ArrayToken { skip, token } => {
                        assert_eq!(45, skip);
                        assert_eq!(6, token.values.len());
                    }
                    _ => panic!("Expected ArrayToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_null() {
        let input = "[null]";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::NullToken { .. } => {}
                        _ => panic!("Expected NullToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_true() {
        let input = "[true]";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::TrueToken { .. } => {}
                        _ => panic!("Expected TrueToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_false() {
        let input = "[false]";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::FalseToken { .. } => {}
                        _ => panic!("Expected FalseToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_number() {
        let input = "[42]";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::NumberToken { .. } => {}
                        _ => panic!("Expected NumberToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_string() {
        let input = r#"["foo"]"#;
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::StringToken { .. } => {}
                        _ => panic!("Expected StringToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_array() {
        let input = "[[1,2]]";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::ArrayToken { .. } => {}
                        _ => panic!("Expected ArrayToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array_with_object() {
        let input = r#"[{"a":1}]"#;
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ArrayToken { token, .. } => {
                    assert_eq!(1, token.values.len());
                    match *token.values[0] {
                        ValueToken::ObjectToken { .. } => {}
                        _ => panic!("Expected ObjectToken"),
                    }
                }
                _ => panic!("Expected ArrayToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }
}
