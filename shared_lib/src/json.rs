use crate::types::{Json, ValueToken};
use crate::value::parse_value;
use lazy_static::lazy_static;
use regex::Regex;

enum Mode {
    Scanning,
    Value,
}

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse(json: &str) -> Result<Json, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;

    loop {
        match json.chars().nth(pos) {
            Some(ch) => {
                match mode {
                    Mode::Scanning => {
                        let char = &ch.to_string()[..];
                        if WHITESPACE.is_match(char) {
                            pos += 1;
                        } else {
                            mode = Mode::Value;
                        }
                    }
                    Mode::Value => {
                        let slice: String = json.chars().skip(pos).collect();
                        match parse_value(&slice, r"[ \n\r\t]") {
                            // None
                            Ok(ValueToken::ArrayToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::ArrayToken { skip, token }),
                                });
                            }
                            Ok(ValueToken::FalseToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::FalseToken { skip, token }),
                                });
                            }
                            Ok(ValueToken::NullToken { skip }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::NullToken { skip }),
                                });
                            }
                            Ok(ValueToken::NumberToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::NumberToken { skip, token }),
                                });
                            }
                            Ok(ValueToken::ObjectToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::ObjectToken { skip, token }),
                                });
                            }
                            Ok(ValueToken::StringToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::StringToken { skip, token }),
                                });
                            }
                            Ok(ValueToken::TrueToken { skip, token }) => {
                                return Ok(Json {
                                    skip: pos + skip,
                                    token: Box::new(ValueToken::TrueToken { skip, token }),
                                });
                            }
                            _ => {
                                return Err("Unexpected token");
                            }
                        }
                    }
                }
            }
            None => {
                return Err("Expected value");
            }
        }
    }
}
