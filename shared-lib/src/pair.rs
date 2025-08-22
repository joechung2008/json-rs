use crate::string::parse_string;
use crate::types::{Pair, ValueToken};
use crate::value::parse_value;
use regex::Regex;

enum Mode {
    Scanning,
    StringValue,
    Delimiter,
    Value,
    End,
}

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_pair(pair: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let mut key = String::new();
    let mut value: Option<ValueToken> = None;

    while let Some(ch) = pair.chars().nth(pos) {
        let char = &ch.to_string()[..];

        match mode {
            Mode::Scanning => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else {
                    mode = Mode::StringValue;
                }
            }
            Mode::StringValue => {
                let slice: String = pair.chars().skip(pos).collect();
                match parse_string(&slice) {
                    Ok(ValueToken::StringToken { skip, token }) => {
                        key = token;
                        pos += skip;
                        mode = Mode::Delimiter;
                    }
                    Ok(_) => {
                        return Err("Expected string");
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Mode::Delimiter => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == ':' {
                    pos += 1;
                    mode = Mode::Value;
                } else {
                    return Err("Expected ':'");
                }
            }
            Mode::Value => {
                let slice: String = pair.chars().skip(pos).collect();
                match parse_value(&slice, r"[ \n\r\t\},]") {
                    Ok(ValueToken::ArrayToken { skip, token }) => {
                        value = Some(ValueToken::ArrayToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::FalseToken { skip, token }) => {
                        value = Some(ValueToken::FalseToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::NullToken { skip }) => {
                        value = Some(ValueToken::NullToken { skip });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::NumberToken { skip, token }) => {
                        value = Some(ValueToken::NumberToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::ObjectToken { skip, token }) => {
                        value = Some(ValueToken::ObjectToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::StringToken { skip, token }) => {
                        value = Some(ValueToken::StringToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    Ok(ValueToken::TrueToken { skip, token }) => {
                        value = Some(ValueToken::TrueToken { skip, token });
                        pos += skip;
                        mode = Mode::End;
                    }
                    _ => {
                        return Err("Unexpected token");
                    }
                }
            }
            Mode::End => break,
        }
    }

    match value {
        Some(value) => Ok(ValueToken::PairToken {
            skip: pos,
            token: Pair {
                key,
                value: Box::new(value),
            },
        }),
        _ => Err("Expected pair token"),
    }
}
