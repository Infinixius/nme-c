#[macro_use]
mod macros;
mod parser;
mod tokenizer;

use colored::Colorize;
use parser::{parse, Context};
use std::time::Instant;
use tokenizer::tokenize;

fn main() {
    if let Some(file_path) = std::env::args().nth(1) {
        let source: String = std::fs::read_to_string(&file_path).unwrap_or_else(|error| {
			errorln!("{}", error);
			std::process::exit(1);
		});

        let start = Instant::now();

        println!("{} Tokenization step", "BEGIN:".yellow());

        let tokens = tokenize(source);

        for token in &tokens {
            println!("{:?}", token);
        }

        println!("{} BEGIN: Parsing step", "BEGIN:".yellow());

        let tree = parse(tokens, Context::Program);

        for node in &tree {
            println!("{:?}", node);
        }

        println!("{} Compiled {} in {:?}", "SUCCESS:".bright_green(), file_path, start.elapsed());
    } else {
        errorln!("No file path provided");
    }
}
