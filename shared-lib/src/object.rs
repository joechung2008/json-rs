use crate::pair::parse_pair;
use crate::types::{Object, Pair, ValueToken};
use regex::Regex;

enum Mode {
    Scanning,
    Pair,
    Delimiter,
    End,
}

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_object(object: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let mut members: Vec<Pair> = Vec::new();

    while let Some(ch) = object.chars().nth(pos) {
        let char = &ch.to_string()[..];

        match mode {
            Mode::Scanning => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '{' {
                    pos += 1;
                    mode = Mode::Pair;
                } else {
                    return Err("Expected '{'");
                }
            }
            Mode::Pair => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '}' {
                    if !members.is_empty() {
                        return Err("Unexpected ','");
                    }
                    pos += 1;
                    mode = Mode::End;
                } else {
                    let slice: String = object.chars().skip(pos).collect();
                    match parse_pair(&slice) {
                        // TODO r"[\s,\}]"
                        Ok(ValueToken::PairToken { skip, token }) => {
                            members.push(token);
                            pos += skip;
                            mode = Mode::Delimiter;
                        }
                        Ok(_) => {
                            return Err("Expected key-value pair");
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            Mode::Delimiter => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == ',' {
                    pos += 1;
                    mode = Mode::Pair;
                } else if ch == '}' {
                    pos += 1;
                    mode = Mode::End;
                } else {
                    return Err("Expected ',' or '}'");
                }
            }
            Mode::End => break,
        }
    }

    Ok(ValueToken::ObjectToken {
        skip: pos,
        token: Object { members },
    })
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::{Json, ValueToken};

    #[test]
    fn test_empty_object() {
        let input = "{}";
        let expected_json_skip = 2;
        let expected_token_skip = 2;
        let expected_members_len = 0;
        match json::parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);

                let unboxed = *token;
                match unboxed {
                    ValueToken::ObjectToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        assert_eq!(expected_members_len, token.members.len());
                    }
                    _ => {
                        panic!("Expected ObjectToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_empty_object_with_spaces() {
        let input = " { } ";
        let expected_json_skip = 4;
        let expected_token_skip = 3;
        let expected_members_len = 0;
        match json::parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);

                let unboxed = *token;
                match unboxed {
                    ValueToken::ObjectToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        assert_eq!(expected_members_len, token.members.len());
                    }
                    _ => {
                        panic!("Expected ObjectToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_array_with_one_of_each_json_type() {
        let input = r#"{"arr":[null,true,false,0,"str",{}]}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let arr_pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "arr")
                        .expect("Missing 'arr' key");
                    match *arr_pair.value {
                        ValueToken::ArrayToken { ref token, .. } => {
                            assert_eq!(token.values.len(), 6);
                            assert!(matches!(*token.values[0], ValueToken::NullToken { .. }));
                            assert!(matches!(*token.values[1], ValueToken::TrueToken { .. }));
                            assert!(matches!(*token.values[2], ValueToken::FalseToken { .. }));
                            match &*token.values[3] {
                                ValueToken::NumberToken { token, .. } => {
                                    assert_eq!(token.value, 0.0)
                                }
                                _ => panic!("Expected NumberToken"),
                            }
                            match &*token.values[4] {
                                ValueToken::StringToken { token, .. } => assert_eq!(token, "str"),
                                _ => panic!("Expected StringToken"),
                            }
                            assert!(matches!(*token.values[5], ValueToken::ObjectToken { .. }));
                        }
                        _ => panic!("Expected ArrayToken"),
                    }
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("Failed to parse: {}", e),
        }
    }

    #[test]
    fn object_with_false_property() {
        let input = r#"{"f":false}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "f")
                        .expect("Missing 'f' key");
                    assert!(matches!(*pair.value, ValueToken::FalseToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_true_property() {
        let input = r#"{"t":true}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "t")
                        .expect("Missing 't' key");
                    assert!(matches!(*pair.value, ValueToken::TrueToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_null_property() {
        let input = r#"{"n":null}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "n")
                        .expect("Missing 'n' key");
                    assert!(matches!(*pair.value, ValueToken::NullToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_array_property() {
        let input = r#"{"a":[1,2,3]}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "a")
                        .expect("Missing 'a' key");
                    assert!(matches!(*pair.value, ValueToken::ArrayToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_object_property() {
        let input = r#"{"o":{"x":1}}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "o")
                        .expect("Missing 'o' key");
                    assert!(matches!(*pair.value, ValueToken::ObjectToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_string_property() {
        let input = r#"{"s":"hello"}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "s")
                        .expect("Missing 's' key");
                    assert!(matches!(*pair.value, ValueToken::StringToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object_with_number_property() {
        let input = r#"{"n":42}"#;
        match json::parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    let pair = token
                        .members
                        .iter()
                        .find(|p| p.key == "n")
                        .expect("Missing 'n' key");
                    assert!(matches!(*pair.value, ValueToken::NumberToken { .. }));
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }
}
