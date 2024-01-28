use crate::lexing::lexer::{Token, TokenKind};

/*

program ::= {statement}
statement ::= "print" (expression | string) sc
    | "if" comparison openCurly {statement} closeCurly sc
    | "WHILE" comparison openCurly nl {statement} closeCurly sc
    | "LET" ident "=" expression sc
    | "INPUT" ident sc
comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
expression ::= term {( "-" | "+" ) term}
term ::= unary {( "/" | "*" ) unary}
unary ::= ["+" | "-"] primary
primary ::= number | ident
sc ::= ';'+
openCurly ::= '{'
closeCurly ::= '}'

*/
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    //Todo, allow less explicit token input
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        println!("PROGRAM");
        while let Some(token) = self.current_token() {
            match token.kind {
                TokenKind::Eof => break,
                _ => self.check_statement(),
            }
            self.advance_token();
        }
    }

    fn check_statement(&mut self) {
        if self.is_current_token(TokenKind::Print) {
            println!("STATEMENT-PRINT");
            self.advance_token();

            if self.is_current_token(TokenKind::LiteralString) {
                self.advance_token(); //TODO get literal
            } else {
                self.check_expression();
            }
            self.check_semicolon();
        }
    }

    fn check_expression(&mut self) {}

    fn check_semicolon(&mut self) {}

    fn is_current_token(&self, to_check_against: TokenKind) -> bool {
        match self.current_token() {
            Some(t) => t.kind == to_check_against,
            None => false,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance_token(&mut self) {
        self.current += 1;
    }

    fn peek_next_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::lexer::TokenData;

    use super::*;

    #[test]
    fn check_if_works() {
        let tokens = vec![
            Token {
                kind: TokenKind::Print,
                data: TokenData { raw: "".into() },
            },
            Token {
                kind: TokenKind::LiteralString,
                data: TokenData {
                    raw: "Hello".into(),
                },
            },
        ];
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert!(false)
    }
}
