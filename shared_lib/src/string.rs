use crate::types::ValueToken;
use regex::Regex;

enum Mode {
    Scanning,
    Character,
    EscapedCharacter,
    Unicode,
    End,
}

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_string(string: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let mut token = String::new();

    while let Some(ch) = string.chars().nth(pos) {
        let char = &ch.to_string()[..];

        match mode {
            Mode::Scanning => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '"' {
                    pos += 1;
                    mode = Mode::Character;
                } else {
                    return Err("Expected '\"'");
                }
            }
            Mode::Character => {
                if ch == '\\' {
                    pos += 1;
                    mode = Mode::EscapedCharacter;
                } else if ch == '"' {
                    pos += 1;
                    mode = Mode::End;
                } else if ch != '\n' && ch != '\r' {
                    pos += 1;
                    token.push_str(char);
                } else {
                    return Err("Unexpected character");
                }
            }
            Mode::EscapedCharacter => {
                if ch == '"' || ch == '\\' || ch == '/' {
                    pos += 1;
                    token.push_str(char);
                    mode = Mode::Character;
                } else if ch == 'b' {
                    pos += 1;
                    token.push('\u{8}'); // no \b in Rust
                    mode = Mode::Character;
                } else if ch == 'f' {
                    pos += 1;
                    token.push('\u{000c}'); // no \f in Rust
                    mode = Mode::Character;
                } else if ch == 'n' {
                    pos += 1;
                    token.push('\n');
                    mode = Mode::Character;
                } else if ch == 'r' {
                    pos += 1;
                    token.push('\r');
                    mode = Mode::Character;
                } else if ch == 't' {
                    pos += 1;
                    token.push('\t');
                    mode = Mode::Character;
                } else if ch == 'u' {
                    mode = Mode::Unicode;
                } else {
                    return Err("Unexpected escape characxter");
                }
            }
            Mode::Unicode => {
                // TODO Make sure slice is a valid hex code
                let slice: String = string.chars().skip(pos).take(4).collect();
                token.push('\\');
                token.push_str(&slice);
                pos += 4;
                mode = Mode::Character;
            }
            Mode::End => break,
        }
    }
    Ok(ValueToken::StringToken { skip: pos, token })
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::{Json, ValueToken};

    #[test]
    fn strings() {
        for &(input, expected_json_skip, expected_token_skip, expected_token) in [
            ("\"Hello, world!\"", 15, 15, "Hello, world!"),
            ("\"\\u0022\"", 8, 8, "\\u0022"),
            ("\"\"", 2, 2, ""),
            (" \"\" ", 3, 2, ""),
        ]
        .iter()
        {
            match json::parse(input) {
                Ok(Json { skip, token }) => {
                    assert_eq!(expected_json_skip, skip);

                    let unboxed = *token;
                    match unboxed {
                        ValueToken::StringToken { skip, token } => {
                            assert_eq!(expected_token_skip, skip);
                            assert_eq!(expected_token, token);
                        }
                        _ => {
                            assert!(false);
                        }
                    }
                }
                Err(e) => {
                    assert!(false, "{}", e);
                }
            }
        }
    }
}
