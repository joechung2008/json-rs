use crate::types::ValueToken;
use regex::Regex;

#[derive(PartialEq)]
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
                    pos += 1;
                    mode = Mode::Unicode;
                } else {
                    return Err("Unexpected escape characxter");
                }
            }
            Mode::Unicode => {
                // Ensure there are at least 4 hex digits
                if string.len() < pos + 4 {
                    return Err("Invalid unicode escape: too short");
                }
                let slice: String = string.chars().skip(pos).take(4).collect();
                if !slice.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err("Invalid unicode escape: non-hex digit");
                }
                // Convert hex to char
                if let Ok(codepoint) = u32::from_str_radix(&slice, 16) {
                    if let Some(unicode_char) = std::char::from_u32(codepoint) {
                        token.push(unicode_char);
                    } else {
                        return Err("Invalid unicode codepoint");
                    }
                } else {
                    return Err("Invalid unicode escape");
                }
                pos += 4;
                mode = Mode::Character;
            }
            Mode::End => break,
        }
    }
    // If we didn't reach Mode::End, string was unterminated
    if mode != Mode::End {
        return Err("Unterminated string");
    }
    Ok(ValueToken::StringToken { skip: pos, token })
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::{Json, ValueToken};

    #[test]
    fn test_hello_world() {
        let input = "\"Hello, world!\"";
        let expected_json_skip = 15;
        let expected_token_skip = 15;
        let expected_token = "Hello, world!";
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
                        panic!("Expected StringToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_escaped_quote() {
        let input = r#""\"""#;
        let expected_json_skip = 4;
        let expected_token_skip = 4;
        let expected_token = "\"";
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
                        panic!("Expected StringToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_empty_string() {
        let input = r#""""#;
        let expected_json_skip = 2;
        let expected_token_skip = 2;
        let expected_token = "";
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
                        panic!("Expected StringToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_empty_string_with_spaces() {
        let input = r#" "" "#;
        let expected_json_skip = 3;
        let expected_token_skip = 2;
        let expected_token = "";
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
                        panic!("Expected StringToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_unicode_escape() {
        let input = "\"\\u0041\"";
        let expected_token = "A";
        match json::parse(input) {
            Ok(Json { token, .. }) => {
                let unboxed = *token;
                match unboxed {
                    ValueToken::StringToken { token, .. } => {
                        assert_eq!(expected_token, token);
                    }
                    _ => panic!("Expected StringToken"),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_invalid_missing_quotes() {
        let input = "foo";
        match json::parse(input) {
            Ok(_) => panic!("Should have failed"),
            Err(_) => {}
        }
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#""unterminated"#;
        match json::parse(input) {
            Ok(_) => panic!("Should have failed"),
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_escape() {
        let input = r#""bad\q""#;
        match json::parse(input) {
            Ok(_) => panic!("Should have failed"),
            Err(_) => {}
        }
    }
}
