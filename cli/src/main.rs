use shared_lib::{Json, ValueToken, parse};
use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    // Parse input using shared_lib::parse
    match parse(&input) {
        Ok(Json { token, .. }) => {
            let pretty = pretty_print_token(&token, 0);
            println!("{}", pretty);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}

// Recursively pretty-prints a ValueToken with indentation
fn pretty_print_token(token: &ValueToken, indent: usize) -> String {
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
