//! The LEXOR Interpreter CLI Core.
//!
//! This module natively boots the Lexer, Parser, and Evaluator instances, linking them
//! cleanly sequentially into a single unified execution pipeline.

use lexor_core::environment::{Environment, EnvironmentIO};
use lexor_core::evaluator::eval_program;
use lexor_core::lexer::Lexer;
use lexor_core::object::Object;
use lexor_core::parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

struct TerminalIO;

impl EnvironmentIO for TerminalIO {
    fn read_line(&mut self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to cleanly read terminal line input");
        input
    }

    fn print(&mut self, text: &str) {
        print!("{}", text);
        io::stdout().flush().unwrap(); // Flush strictly ensures immediate render
    }
}

fn main() {
    // 1. Gather command line arguments ensuring a `.lexor` file was explicitly provided
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <file.lexor>");
        process::exit(1);
    }

    let filename = &args[1];

    // 2. Read the source code directly natively buffering from the `.lexor` file
    let source_code = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Error securely reading file {}: {}", filename, err);
        process::exit(1);
    });

    println!("--- Executing LEXOR Script: {} ---\n", filename);

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Some(program) => {
            if !parser.errors.is_empty() {
                println!("--- FATAL SYNTAX ERRORS ---");
                for err in &parser.errors {
                    println!("Error: {}", err);
                }
            } else {
                let mut env = Environment::new();
                let mut io = TerminalIO;
                let result = eval_program(&program, &mut env, &mut io);

                if let Some(Object::Error(msg)) = result {
                    println!("\n--- FATAL RUNTIME ERROR ---");
                    println!("{}", msg);
                } else {
                    println!("\n--- Program finished executing. ---");
                }
            }
        }
        None => {
            println!("--- FATAL PARSING FAILURE ---");
            for err in parser.errors {
                println!("Error: {}", err);
            }
        }
    }
}
