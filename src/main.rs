mod parser;
mod tokenizer;

use parser::parse;
use std::time::Instant;
use tokenizer::tokenize;

fn main() {
    if let Some(file_path) = std::env::args().nth(1) {
        let source: String = std::fs::read_to_string(&file_path).unwrap_or_else(|error| {
			eprintln!("ERROR: {}", error);
			std::process::exit(1);
		});

        let start = Instant::now();

        println!("BEGIN: Tokenization step");

        let tokens = tokenize(source);

        for token in &tokens {
            println!("{:?}", token);
        }

        println!("BEGIN: Parsing step");

        let tree = parse(tokens);

        for node in &tree {
            println!("{:?}", node);
        }

        println!("SUCCESS: Compiled {} in {:?}", file_path, start.elapsed());
    } else {
        println!("ERROR: No file path provided");
    }
}
