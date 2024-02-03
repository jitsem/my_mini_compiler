use std::fs;

use crate::{
    emitting::emitter::CEmitter,
    lexing::lexer::{Lexer, Token, TokenKind},
    parsing::parser::Parser,
};
const INPUT: &str = include_str!("../data/example1.scrpt");
mod emitting;
mod lexing;
mod parsing;
fn main() {
    println!("Start Compiling!");
    println!("Lexing input!");
    let lex = Lexer::from(INPUT);
    let tokens = lex.into_iter().collect::<Vec<Token>>();
    for t in tokens.iter() {
        if t.kind == TokenKind::Invalid {
            eprintln!("Invalid TOKEN: {}", t.data.raw);
            return;
        }
    }
    println!("Done lexing!");
    println!("Parsing tokens!");
    let parser = Parser::new(&tokens);
    let res = parser.parse();
    match res {
        Ok(statements) => {
            println!("Done Parsing!\n");
            let emitter = CEmitter::new(&statements);
            let code = emitter.emit();
            println!("Writing to test.c");
            fs::write("target/debug/test.c", code).expect("Unable to write file");
        }
        Err(e) => {
            eprintln!("Unexpected token:{:?} ; {:?}", e.token, e.reason)
        }
    }
}
