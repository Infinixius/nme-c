use colored::Colorize;

mod compiler;
#[macro_use]
mod macros;
mod parser;
mod tokenizer;
use parser::{parse, Context};
use panic_message::get_panic_info_message;
use std::time::Instant;
use tokenizer::tokenize;

fn main() {
    // Setup custom panic hook
    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| {
        if panic_info.location().is_some() && get_panic_info_message(panic_info).is_some() {
            let location = panic_info.location().unwrap();
            println!(
                "{} {:?} at {} line {}:{}",

                "PANIC:".red(),
                get_panic_info_message(panic_info).unwrap(),
                location.file(),
                location.line(),
                location.column()
            );
        } else {
            println!("{} {:?}", "PANIC:".red(), panic_info);
        }
		std::process::exit(1);
	}));

    // Begin actual program by checking if a file path was provided
    if let Some(file_path) = std::env::args().nth(1) {
        let source: String = std::fs::read_to_string(&file_path).unwrap_or_else(|error| {
			errorln!("{}", error);
			std::process::exit(1);
		});

        let start = Instant::now();

        println!("{} Tokenization step", "BEGIN:".yellow());

        let tokens = tokenize(&source);

        for token in &tokens {
            println!("{:?}", token);
        }

        println!("{} Parsing step", "BEGIN:".yellow());

        let tree = parse(tokens, Context::Program);

        for node in &tree {
            println!("{:?}", node);
        }

        println!("{} Compilation step", "BEGIN:".yellow());

        let instructions = compiler::compile(tree);

        for instruction in &instructions {
            println!("{:?}", instruction);
        }

        println!("{} Compiled {} in {:?}", "SUCCESS:".bright_green(), file_path, start.elapsed());
    } else {
        errorln!("No file path provided");
    }
}
