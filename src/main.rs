pub mod tokens;
use tokens::Token;

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
