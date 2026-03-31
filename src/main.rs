pub mod tokens;
pub mod lexer;

use lexer::Lexer;

fn main() {
    let source_code = r#"
SCRIPT
    %% This is a quick test of the LEXOR language scanner.
    DECLARE INT length = 100
    DECLARE INT width = 50
    DECLARE INT area = length * width
    
    PRINT "The area is: ", area, $
END
"#;

    println!("--- Scanning Source Code ---");
    println!("{}", source_code.trim());
    println!("----------------------------\n");
    
    let lexer = Lexer::new(source_code);

    // Because we implemented `Iterator` for our Lexer, 
    // we can easily iterate directly over the tokens.
    for token in lexer {
        println!("{:?}", token);
    }
}
