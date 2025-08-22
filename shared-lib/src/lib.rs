#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use json::parse;
pub use types::{Json, ValueToken};

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
}
