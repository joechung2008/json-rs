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

pub fn pretty_print_token(token: &ValueToken, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match token {
        ValueToken::ArrayToken { skip, token: array } => {
            let mut s = format!("ArrayToken (skip: {}) [\n", skip);
            for (i, v) in array.values.iter().enumerate() {
                s.push_str(&indent_str);
                s.push_str("  ");
                s.push_str(&pretty_print_token(v, indent + 1));
                if i < array.values.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&indent_str);
            s.push(']');
            s
        }
        ValueToken::ObjectToken {
            skip,
            token: object,
        } => {
            let mut s = format!("ObjectToken (skip: {}) {{\n", skip);
            for (i, pair) in object.members.iter().enumerate() {
                s.push_str(&indent_str);
                s.push_str("  ");
                s.push_str(&format!(
                    "\"{}\": {}",
                    pair.key,
                    pretty_print_token(&pair.value, indent + 1)
                ));
                if i < object.members.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&indent_str);
            s.push('}');
            s
        }
        ValueToken::StringToken {
            skip,
            token: string,
        } => format!("StringToken (skip: {}) \"{}\"", skip, string),
        ValueToken::NumberToken {
            skip,
            token: number,
        } => format!("NumberToken (skip: {}) {}", skip, number.value_as_string),
        ValueToken::TrueToken { skip, token } => format!("TrueToken (skip: {}) {}", skip, token),
        ValueToken::FalseToken { skip, token } => format!("FalseToken (skip: {}) {}", skip, token),
        ValueToken::NullToken { skip } => format!("NullToken (skip: {})", skip),
        ValueToken::PairToken { skip, token: pair } => {
            format!(
                "PairToken (skip: {}) \"{}\": {}",
                skip,
                pair.key,
                pretty_print_token(&pair.value, indent + 1)
            )
        }
    }
}
