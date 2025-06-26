use rust_sql::parser;
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match parser::parse_sql(input) {
            Ok(statement) => {
                println!("Parsed successfully:");
                println!("{:#?}", statement);
            }
            Err(e) => {
                eprintln!("Parse failed: {}", e);
            }
        }
    }
}
