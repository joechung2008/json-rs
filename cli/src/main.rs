use shared_lib::{Json, parse, pretty_print_token};
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
