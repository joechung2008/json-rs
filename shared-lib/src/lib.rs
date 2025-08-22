#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use json::parse;
pub use types::{Json, ValueToken};

pub fn pretty_print_token(token: &ValueToken, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match token {
        ValueToken::ArrayToken { skip, token: array } => {
            let mut s = format!("ArrayToken (skip: {}) [\n", skip);
            for (i, v) in array.values.iter().enumerate() {
                s.push_str(&indent_str);
                s.push_str("  ");
                s.push_str(&pretty_print_token(v, indent + 1));
                if i < array.values.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&indent_str);
            s.push(']');
            s
        }
        ValueToken::ObjectToken {
            skip,
            token: object,
        } => {
            let mut s = format!("ObjectToken (skip: {}) {{\n", skip);
            for (i, pair) in object.members.iter().enumerate() {
                s.push_str(&indent_str);
                s.push_str("  ");
                s.push_str(&format!(
                    "\"{}\": {}",
                    pair.key,
                    pretty_print_token(&pair.value, indent + 1)
                ));
                if i < object.members.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&indent_str);
            s.push('}');
            s
        }
        ValueToken::StringToken {
            skip,
            token: string,
        } => format!("StringToken (skip: {}) \"{}\"", skip, string),
        ValueToken::NumberToken {
            skip,
            token: number,
        } => format!("NumberToken (skip: {}) {}", skip, number.value_as_string),
        ValueToken::TrueToken { skip, token } => format!("TrueToken (skip: {}) {}", skip, token),
        ValueToken::FalseToken { skip, token } => format!("FalseToken (skip: {}) {}", skip, token),
        ValueToken::NullToken { skip } => format!("NullToken (skip: {})", skip),
        ValueToken::PairToken { skip, token: pair } => {
            format!(
                "PairToken (skip: {}) \"{}\": {}",
                skip,
                pair.key,
                pretty_print_token(&pair.value, indent + 1)
            )
        }
    }
}

mod array;
mod json;
mod number;
mod object;
mod pair;
mod string;
mod types;
mod value;

#[cfg(test)]
mod tests {
    use crate::Json;
    use crate::ValueToken;
    use crate::parse;

    #[test]
    fn false_test() {
        match parse("false") {
            Ok(Json { skip, token }) => {
                assert_eq!(skip, 5);

                match *token {
                    ValueToken::FalseToken { skip, token } => {
                        assert_eq!(skip, 5);
                        assert!(!token);
                    }
                    _ => {
                        panic!("Expected FalseToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn null_test() {
        match parse("null") {
            Ok(Json { skip, token }) => {
                assert_eq!(skip, 4);

                match *token {
                    ValueToken::NullToken { skip } => {
                        assert_eq!(skip, 4);
                    }
                    _ => {
                        panic!("Expected NullToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn true_test() {
        match parse("true") {
            Ok(Json { skip, token }) => {
                assert_eq!(skip, 4);

                match *token {
                    ValueToken::TrueToken { skip, token } => {
                        assert_eq!(skip, 4);
                        assert!(token);
                    }
                    _ => {
                        panic!("Expected TrueToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn object_with_number_test() {
        match parse(r#"{"n":0}"#) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::ObjectToken { ref token, .. } => {
                    assert_eq!(token.members.len(), 1);
                    let pair = &token.members[0];
                    assert_eq!(pair.key, "n");
                    match *pair.value {
                        ValueToken::NumberToken { ref token, .. } => {
                            assert_eq!(token.value_as_string, "0");
                        }
                        _ => panic!("Expected NumberToken for value"),
                    }
                }
                _ => panic!("Expected ObjectToken"),
            },
            Err(e) => {
                panic!("Failed to parse object with number: {}", e);
            }
        }
    }
}
