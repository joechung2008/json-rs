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
    fn arrays() {
        for &(input, expected_json_skip, expected_token_skip, expected_values_len) in [
            ("[]", 2, 2, 0),
            (" [ ] ", 4, 3, 0),
            ("[[]]", 4, 4, 1),
            (" [ [ ] ] ", 8, 7, 1),
            ("[[], false]", 11, 11, 2),
            ("[[], false, null]", 17, 17, 3),
            ("[[], false, null, 1.2e3]", 24, 24, 4),
            ("[[], false, null, 1.2e3, {}]", 28, 28, 5),
            ("[[], false, null, 1.2e3, {}, \"Hello, world!\"]", 45, 45, 6),
        ]
        .iter()
        {
            match parse(input) {
                Ok(Json { skip, token }) => {
                    assert_eq!(expected_json_skip, skip);

                    let unboxed = *token;
                    match unboxed {
                        ValueToken::ArrayToken { skip, token } => {
                            assert_eq!(expected_token_skip, skip);
                            assert_eq!(expected_values_len, token.values.len());
                        }
                        _ => {
                            panic!("Expected ArrayToken");
                        }
                    }
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
}
