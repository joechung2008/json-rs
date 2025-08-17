use crate::array::parse_array;
use crate::number::parse_number;
use crate::object::parse_object;
use crate::string::parse_string;
use crate::types::ValueToken;
use regex::Regex;

enum Mode {
    Scanning,
    Array,
    False,
    Null,
    Number,
    Object,
    StringValue,
    True,
    End,
}

lazy_static! {
    static ref DIGIT_OR_DASH: Regex = Regex::new(r"[-\d]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_value(value: &str, delimiters: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let is_delimiter: &Regex = &Regex::new(delimiters).unwrap();

    while let Some(ch) = value.chars().nth(pos) {
        match mode {
            Mode::Scanning => {
                let char = &ch.to_string()[..];

                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '[' {
                    mode = Mode::Array;
                } else if ch == 'f' {
                    mode = Mode::False;
                } else if ch == 'n' {
                    mode = Mode::Null;
                } else if DIGIT_OR_DASH.is_match(char) {
                    mode = Mode::Number;
                } else if ch == '{' {
                    mode = Mode::Object;
                } else if ch == '"' {
                    mode = Mode::StringValue;
                } else if ch == 't' {
                    mode = Mode::True;
                } else if !delimiters.is_empty() && is_delimiter.is_match(char) {
                    mode = Mode::End;
                } else {
                    return Err("Unexpected character");
                }
            }
            Mode::Array => {
                let slice: String = value.chars().skip(pos).collect();
                match parse_array(&slice) {
                    Ok(ValueToken::ArrayToken { skip, token }) => {
                        return Ok(ValueToken::ArrayToken {
                            skip: pos + skip,
                            token,
                        });
                    }
                    Ok(_) => {
                        return Err("Expected array");
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Mode::False => {
                let slice: String = value.chars().skip(pos).take(5).collect();
                if &slice == "false" {
                    return Ok(ValueToken::FalseToken {
                        skip: pos + 5,
                        token: false,
                    });
                } else {
                    return Err("Expected 'false'");
                }
            }
            Mode::Null => {
                let slice: String = value.chars().skip(pos).take(4).collect();
                if &slice == "null" {
                    return Ok(ValueToken::NullToken { skip: pos + 4 });
                } else {
                    return Err("Expected 'null'");
                }
            }
            Mode::Number => {
                let slice: String = value.chars().skip(pos).collect();
                match parse_number(&slice, delimiters) {
                    Ok(ValueToken::NumberToken { skip, token }) => {
                        return Ok(ValueToken::NumberToken {
                            skip: pos + skip,
                            token,
                        });
                    }
                    Ok(_) => {
                        return Err("Expected number");
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Mode::Object => {
                let slice: String = value.chars().skip(pos).collect();
                match parse_object(&slice) {
                    Ok(ValueToken::ObjectToken { skip, token }) => {
                        return Ok(ValueToken::ObjectToken {
                            skip: pos + skip,
                            token,
                        });
                    }
                    Ok(_) => {
                        return Err("Expected number");
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Mode::StringValue => {
                let slice: String = value.chars().skip(pos).collect();
                match parse_string(&slice) {
                    Ok(ValueToken::StringToken { skip, token }) => {
                        return Ok(ValueToken::StringToken {
                            skip: pos + skip,
                            token,
                        });
                    }
                    Ok(_) => {
                        return Err("Expected string");
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Mode::True => {
                let slice: String = value.chars().skip(pos).take(4).collect();
                if &slice == "true" {
                    return Ok(ValueToken::TrueToken {
                        skip: pos + 4,
                        token: true,
                    });
                } else {
                    return Err("Expected 'true'");
                }
            }
            Mode::End => break,
        }
    }

    Err("Expected value token")
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::{Json, ValueToken};

    #[test]
    fn test_true() {
        match json::parse("true") {
            Ok(Json { skip, token }) => {
                assert_eq!(4, skip);

                let unboxed = *token;
                match unboxed {
                    ValueToken::TrueToken { skip, token } => {
                        assert_eq!(4, skip);
                        assert_eq!(true, token);
                    }
                    _ => {
                        assert!(false, "Expected true token");
                    }
                }
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_false() {
        match json::parse("false") {
            Ok(Json { skip, token }) => {
                assert_eq!(5, skip);

                let unboxed = *token;
                match unboxed {
                    ValueToken::FalseToken { skip, token } => {
                        assert_eq!(5, skip);
                        assert_eq!(false, token);
                    }
                    _ => {
                        assert!(false, "Expected false token");
                    }
                }
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_null() {
        match json::parse("null") {
            Ok(Json { skip, token }) => {
                assert_eq!(4, skip);

                let unboxed = *token;
                match unboxed {
                    ValueToken::NullToken { skip } => {
                        assert_eq!(4, skip);
                    }
                    _ => {
                        assert!(false, "Expected null token");
                    }
                }
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_invalid_json() {
        match json::parse("invalid") {
            Ok(_) => assert!(false, "Expected error for invalid input"),
            Err(_) => {}
        }
    }
}
