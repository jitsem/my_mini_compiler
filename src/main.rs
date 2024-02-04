use std::fs;

use crate::{
    emitting::emitter::CEmitter,
    lexing::lexer::{Lexer, Token, TokenKind},
    parsing::parser::Parser,
};
mod emitting;
mod lexing;
mod parsing;
use clap::{Arg, Command};
use std::process;

fn main() {
    let matches = Command::new("mmc")
        .version("1.0")
        .about("My mini compiler which compiles to C.")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Writes output to a file"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .default_value("C")
                .help("Sets the target language for the output, defaults to C"),
        )
        .arg(
            Arg::new("input")
                .help("Sets the input file to use")
                .value_name("INPUT")
                .required(true)
                .index(1),
        )
        .get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    if target != "C" {
        eprintln!("Currently, only 'C' target is supported.");
        process::exit(1);
    }

    let input_file = matches.get_one::<String>("input").unwrap();

    let input = fs::read_to_string(input_file).expect("Unable to read input file");

    let lex = Lexer::from(&input);
    let tokens: Vec<Token> = lex.into_iter().collect();

    let invalid_tokens: Vec<&Token> = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Invalid)
        .collect();

    if !invalid_tokens.is_empty() {
        eprintln!("Found invalid tokens during lexing.");
        eprintln!("{:?}", invalid_tokens);
        process::exit(1);
    }

    let parser = Parser::new(&tokens);
    let parse_result = parser.parse();

    match parse_result {
        Ok(statements) => {
            let emitter = CEmitter::new(&statements);
            let code = emitter.emit();

            if let Some(output_file) = matches.get_one::<String>("output") {
                fs::write(output_file, code).expect("Unable to write output file");
                println!("Code written to {}", output_file);
            } else {
                println!("{}", code);
            }
        }
        Err(e) => {
            eprintln!(
                "Parsing error: Unexpected token:{:?}; Reason: {:?}",
                e.token, e.reason
            );
            process::exit(1);
        }
    }
}
