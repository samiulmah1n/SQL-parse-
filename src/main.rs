mod token;
mod tokenizer;
mod statement;
mod parser;

use std::io::{self, Write};
use parser::build_statement;

fn main() {
    println!("SQL Parser CLI. Type your SQL statements below:");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }
        match build_statement(&input) {
            Ok(statement) => println!("Parsed statement: {:?}", statement),
            Err(e) => println!("Error: {}", e),
        }
    }
}