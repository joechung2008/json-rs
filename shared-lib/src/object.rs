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
    fn objects() {
        for &(input, expected_json_skip, expected_token_skip, expected_members_len) in
            [("{}", 2, 2, 0), (" { } ", 4, 3, 0)].iter()
        {
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
    }
}
