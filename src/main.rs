pub mod tokens;
pub mod lexer;

use tokens::Token;
use lexer::Lexer;

fn main() {
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::IntLit(42),
        Token::Dollar,
    ];

    for token in tokens {
        println!("{:?}", token);
    }
}
