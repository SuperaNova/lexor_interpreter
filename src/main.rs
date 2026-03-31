pub mod tokens;
pub mod lexer;
pub mod ast;
pub mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source_code = r#"
SCRIPT AREA
START SCRIPT
    %% This is a parser test
    DECLARE INT length = 100
    DECLARE INT width = 50
    DECLARE INT area = length * width
    
    PRINT: "The area is: " & area & $
END SCRIPT
"#;

    println!("--- Parsing Source Code ---");
    println!("{}", source_code.trim());
    println!("----------------------------\n");
    
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Some(program) => {
            println!("--- AST Construction Results ---");
            if !parser.errors.is_empty() {
                println!("Parsed with Syntax Errors:");
                for err in &parser.errors {
                    println!("Error: {}", err);
                }
            } else {
                println!("Successfully built AST without errors!\n");
                for statement in program.statements {
                    // We use {:#?} for beautiful pretty-printing of the nested syntax trees
                    println!("{:#?}", statement);
                }
            }
        },
        None => {
            println!("--- FATAL PARSING FAILURE ---");
            for err in parser.errors {
                println!("Error: {}", err);
            }
        }
    }
}
