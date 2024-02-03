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
        println!("{:?}", t.kind);
        if t.kind == TokenKind::Invalid {
            eprintln!("Invalid TOKEN: {}", t.data.raw)
        }
    }
    println!("Done lexing!");
    println!("Parsing tokens!");
    let parser = Parser::new(&tokens);
    let res = parser.parse();
    match res {
        Ok(statements) => {
            println!("Done Parsing!");
            println!("{:#?}", statements);
            let emitter = CEmitter::new(&statements);
            let code = emitter.emit();
            println!("{}", code);
        }
        Err(e) => {
            eprintln!("Unexpected token:{:?} ; {:?}", e.token, e.reason)
        }
    }
}
