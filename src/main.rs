use crate::lexing::lexer::{Lexer, TokenKind};
const INPUT: &str = include_str!("../data/example1.scrpt");
mod lexing;
fn main() {
    println!("Analzing input!");
    println!("Analzing input!");
    let mut lex = Lexer::from(INPUT);
    loop {
        let token = lex.next_token().unwrap();
        println!("{:?}", token.kind);
        if token.kind == TokenKind::Unexpected {
            eprintln!("UNEXPECTED TOKEN: {}", token.data.raw)
        }
        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
