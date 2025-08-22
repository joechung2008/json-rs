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
    fn numbers() {
        for &(
            input,
            expected_json_skip,
            expected_token_skip,
            expected_value,
            expected_value_as_string,
        ) in [
            ("0", 1, 1, 0.0, "0"),
            ("-1", 2, 2, -1.0, "-1"),
            ("1", 1, 1, 1.0, "1"),
            (" 1.2e3 ", 6, 5, 1200.0, "1.2e3"),
        ]
        .iter()
        {
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
    }
}
