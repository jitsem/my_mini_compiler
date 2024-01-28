use crate::lexing::lexer::{Lexer, TokenKind};
const INPUT: &str = include_str!("../data/example1.scrpt");
mod lexing;
mod parsing;
fn main() {
    println!("Analzing input!");
    println!("Analzing input!");
    let lex = Lexer::from(INPUT);
    let tokens = lex.into_iter();
    for t in tokens {
        println!("{:?}", t.kind);
        if t.kind == TokenKind::Invalid {
            eprintln!("Invalid TOKEN: {}", t.data.raw)
        }
    }
    println!("Done!");
}
