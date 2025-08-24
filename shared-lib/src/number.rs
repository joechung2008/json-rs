use crate::types::{Number, ValueToken};
use regex::Regex;

enum Mode {
    Scanning,
    Characteristic,
    CharacteristicDigit,
    DecimalPoint,
    Mantissa,
    Exponent,
    ExponentSign,
    ExponentFirstDigit,
    ExponentDigits,
    End,
}

lazy_static! {
    static ref DIGIT: Regex = Regex::new(r"\d").unwrap();
    static ref NON_ZERO_DIGIT: Regex = Regex::new(r"[1-9]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]").unwrap();
}

pub fn parse_number(number: &str, delimiters: &str) -> Result<ValueToken, &'static str> {
    let mut mode = Mode::Scanning;
    let mut pos: usize = 0;
    let mut value_as_string = String::new();
    let is_delimiter: &Regex = &Regex::new(delimiters).unwrap();

    while let Some(ch) = number.chars().nth(pos) {
        let char = &ch.to_string()[..];

        match mode {
            Mode::Scanning => {
                if WHITESPACE.is_match(char) {
                    pos += 1;
                } else if ch == '-' {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::Characteristic;
                } else {
                    mode = Mode::Characteristic;
                }
            }
            Mode::Characteristic => {
                if ch == '0' {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::DecimalPoint;
                } else if NON_ZERO_DIGIT.is_match(char) {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::CharacteristicDigit;
                } else {
                    return Err("Expected digit");
                }
            }
            Mode::CharacteristicDigit => {
                if DIGIT.is_match(char) {
                    value_as_string.push(ch);
                    pos += 1;
                } else if !delimiters.is_empty() && is_delimiter.is_match(char) {
                    mode = Mode::End;
                } else {
                    mode = Mode::DecimalPoint;
                }
            }
            Mode::DecimalPoint => {
                if ch == '.' {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::Mantissa;
                } else if !delimiters.is_empty() && is_delimiter.is_match(char) {
                    mode = Mode::End;
                } else {
                    mode = Mode::Exponent;
                }
            }
            Mode::Mantissa => {
                if DIGIT.is_match(char) {
                    value_as_string.push(ch);
                    pos += 1;
                } else if ch == 'e' || ch == 'E' {
                    mode = Mode::Exponent;
                } else if !delimiters.is_empty() && is_delimiter.is_match(char) {
                    mode = Mode::End;
                } else {
                    return Err("Unexpected character");
                }
            }
            Mode::Exponent => {
                if ch == 'e' || ch == 'E' {
                    value_as_string.push('e');
                    pos += 1;
                    mode = Mode::ExponentSign;
                } else {
                    return Err("Expected 'e' or 'E'");
                }
            }
            Mode::ExponentSign => {
                if ch == '+' || ch == '-' {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::ExponentFirstDigit;
                } else {
                    mode = Mode::ExponentFirstDigit;
                }
            }
            Mode::ExponentFirstDigit => {
                if DIGIT.is_match(char) {
                    value_as_string.push(ch);
                    pos += 1;
                    mode = Mode::ExponentDigits;
                } else {
                    return Err("Expected digit");
                }
            }
            Mode::ExponentDigits => {
                if DIGIT.is_match(char) {
                    value_as_string.push(ch);
                    pos += 1;
                } else if !delimiters.is_empty() && is_delimiter.is_match(char) {
                    mode = Mode::End;
                } else {
                    return Err("Expected digit");
                }
            }
            Mode::End => break,
        }
    }

    match mode {
        Mode::Characteristic | Mode::ExponentFirstDigit | Mode::ExponentSign => {
            return Err("Incomplete expression");
        }
        _ => {}
    }

    Ok(ValueToken::NumberToken {
        skip: pos,
        token: Number {
            value: value_as_string.parse::<f64>().unwrap(),
            value_as_string,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::json::parse;
    use crate::types::{Json, ValueToken};

    #[test]
    fn test_zero() {
        let input = "0";
        let expected_json_skip = 1;
        let expected_token_skip = 1;
        let expected_value = 0.0;
        let expected_value_as_string = "0";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::NumberToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        let epsilon = 1e-10;
                        assert!((expected_value - token.value).abs() < epsilon);
                        assert_eq!(expected_value_as_string, token.value_as_string);
                    }
                    _ => {
                        panic!("Expected NumberToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_negative_one() {
        let input = "-1";
        let expected_json_skip = 2;
        let expected_token_skip = 2;
        let expected_value = -1.0;
        let expected_value_as_string = "-1";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::NumberToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        let epsilon = 1e-10;
                        assert!((expected_value - token.value).abs() < epsilon);
                        assert_eq!(expected_value_as_string, token.value_as_string);
                    }
                    _ => {
                        panic!("Expected NumberToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_one() {
        let input = "1";
        let expected_json_skip = 1;
        let expected_token_skip = 1;
        let expected_value = 1.0;
        let expected_value_as_string = "1";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::NumberToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        let epsilon = 1e-10;
                        assert!((expected_value - token.value).abs() < epsilon);
                        assert_eq!(expected_value_as_string, token.value_as_string);
                    }
                    _ => {
                        panic!("Expected NumberToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_float_exponent() {
        let input = " 1.2e3 ";
        let expected_json_skip = 6;
        let expected_token_skip = 5;
        let expected_value = 1200.0;
        let expected_value_as_string = "1.2e3";
        match parse(input) {
            Ok(Json { skip, token }) => {
                assert_eq!(expected_json_skip, skip);
                let unboxed = *token;
                match unboxed {
                    ValueToken::NumberToken { skip, token } => {
                        assert_eq!(expected_token_skip, skip);
                        let epsilon = 1e-10;
                        assert!((expected_value - token.value).abs() < epsilon);
                        assert_eq!(expected_value_as_string, token.value_as_string);
                    }
                    _ => {
                        panic!("Expected NumberToken");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_plus_sign() {
        let input = "+42";
        let expected_value = 42.0;
        let expected_value_as_string = "42"; // Rust f64 parsing ignores '+'
        if let Ok(Json { token, .. }) = parse(input) {
            if let ValueToken::NumberToken { token, .. } = *token {
                let epsilon = 1e-10;
                assert!((expected_value - token.value).abs() < epsilon);
                assert_eq!(expected_value_as_string, token.value_as_string);
            } else {
                panic!("Expected NumberToken");
            }
        }
    }

    #[test]
    fn test_leading_zero() {
        let input = "012";
        if parse(input).is_ok() {
            panic!("Should not parse number with leading zero");
        }
    }

    #[test]
    fn test_decimal_point_no_mantissa() {
        let input = "2.";
        let expected_value = 2.0;
        let expected_value_as_string = "2.";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::NumberToken { token, .. } => {
                    let epsilon = 1e-10;
                    assert!((expected_value - token.value).abs() < epsilon);
                    assert_eq!(expected_value_as_string, token.value_as_string);
                }
                _ => panic!("Expected NumberToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_decimal_point_and_mantissa() {
        let input = "2.5";
        let expected_value = 2.5;
        let expected_value_as_string = "2.5";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::NumberToken { token, .. } => {
                    let epsilon = 1e-10;
                    assert!((expected_value - token.value).abs() < epsilon);
                    assert_eq!(expected_value_as_string, token.value_as_string);
                }
                _ => panic!("Expected NumberToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_positive_exponent_no_plus() {
        let input = "1e2";
        let expected_value = 100.0;
        let expected_value_as_string = "1e2";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::NumberToken { token, .. } => {
                    let epsilon = 1e-10;
                    assert!((expected_value - token.value).abs() < epsilon);
                    assert_eq!(expected_value_as_string, token.value_as_string);
                }
                _ => panic!("Expected NumberToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_positive_exponent_with_plus() {
        let input = "1e+2";
        let expected_value = 100.0;
        let expected_value_as_string = "1e+2";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::NumberToken { token, .. } => {
                    let epsilon = 1e-10;
                    assert!((expected_value - token.value).abs() < epsilon);
                    assert_eq!(expected_value_as_string, token.value_as_string);
                }
                _ => panic!("Expected NumberToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_negative_exponent() {
        let input = "1e-2";
        let expected_value = 0.01;
        let expected_value_as_string = "1e-2";
        match parse(input) {
            Ok(Json { token, .. }) => match *token {
                ValueToken::NumberToken { token, .. } => {
                    let epsilon = 1e-10;
                    assert!((expected_value - token.value).abs() < epsilon);
                    assert_eq!(expected_value_as_string, token.value_as_string);
                }
                _ => panic!("Expected NumberToken"),
            },
            Err(e) => panic!("{}", e),
        }
    }
}
