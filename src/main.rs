use crate::lexing::lexer::{Lexer, TokenKind};
const INPUT: &str = include_str!("../data/example1.scrpt");
mod lexing;
fn main() {
    println!("Analzing input!");
    println!("Analzing input!");
    let lex = Lexer::from(INPUT);
    let mut tokens = lex.into_iter();
    while let Some(t) = tokens.next() {
        println!("{:?}", t.kind);
        if t.kind == TokenKind::Invalid {
            eprintln!("Invalid TOKEN: {}", t.data.raw)
        }
    }
    println!("Done!");
}
