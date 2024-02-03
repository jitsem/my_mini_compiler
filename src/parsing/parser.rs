use std::collections::HashSet;

use crate::lexing::lexer::{Token, TokenKind};

pub type ParserResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub token: Option<Token>,
    pub expected: Option<TokenKind>,
    pub reason: Option<String>,
}
/*

program ::= {statement}
statement ::= "print" (expression | string) sc
    | "if" comparison openCurly {statement} closeCurly
    | "while" comparison openCurly {statement} closeCurly
    | "let" ident "=" expression sc
    | "input" ident sc
    | ident "=" expression sc
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
    identifiers: HashSet<String>,
}

impl Parser {
    //Todo, allow less explicit token input
    pub fn new(tokens: &Vec<Token>) -> Self {
        let tokens: Vec<Token> = tokens
            .iter()
            .cloned()
            .filter(|t| t.kind != TokenKind::Whitespace)
            .collect();
        let identifiers: HashSet<String> = HashSet::new();
        Self {
            tokens,
            current: 0,
            identifiers,
        }
    }

    pub fn parse(&mut self) -> ParserResult<()> {
        println!("PROGRAM");
        while let Some(token) = self.current_token() {
            match token.kind {
                TokenKind::Eof => break,
                _ => self.match_statement()?,
            }
        }

        Ok(())
    }

    fn match_statement(&mut self) -> ParserResult<()> {
        if self.is_current_token(TokenKind::Print) {
            println!("STATEMENT-PRINT");
            self.advance_token();

            if self.is_current_token(TokenKind::LiteralString) {
                self.advance_token(); //TODO get literal
            } else {
                self.match_expression()?;
            }
            self.match_token(TokenKind::SemiColon)?;
        } else if self.is_current_token(TokenKind::If) {
            println!("STATEMENT-IF");
            self.advance_token();

            self.match_comparison()?;
            self.match_token(TokenKind::OpenCurly)?;
            self.match_statement()?;
            while !self.is_current_token(TokenKind::CloseCurly) {
                self.match_statement()?;
            }
            self.match_token(TokenKind::CloseCurly)?;
        } else if self.is_current_token(TokenKind::While) {
            println!("STATEMENT-WHILE");
            self.advance_token();

            self.match_comparison()?;
            self.match_token(TokenKind::OpenCurly)?;
            self.match_statement()?;
            while !self.is_current_token(TokenKind::CloseCurly) {
                self.match_statement()?;
            }
            self.match_token(TokenKind::CloseCurly)?;
        } else if self.is_current_token(TokenKind::Let) {
            println!("STATEMENT-LET");
            self.advance_token();
            let id = self.match_identifier()?;
            if self.identifiers.contains(&id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is already declared", id)),
                });
            } else {
                self.identifiers.insert(id);
            }
            self.match_token(TokenKind::Equals)?;
            self.match_expression()?;
            self.match_token(TokenKind::SemiColon)?;
        } else if self.is_current_token(TokenKind::Input) {
            println!("STATEMENT-INPUT");
            self.advance_token();
            let id = self.match_identifier()?;
            if self.identifiers.contains(&id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is already declared", id)),
                });
            } else {
                self.identifiers.insert(id);
            }
            self.match_token(TokenKind::SemiColon)?;
        } else if self.is_current_token(TokenKind::Identifier) {
            println!("STATEMENT-Assign");
            let id = self.match_identifier()?;
            if !self.identifiers.contains(&id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is never declared", id)),
                });
            }
            self.match_token(TokenKind::Equals)?;
            self.match_expression()?;
            self.match_token(TokenKind::SemiColon)?;
        } else {
            return Err(ParserError {
                token: self.current_token().cloned(),
                expected: None,
                reason: Some("Unknown statement".to_string()),
            });
        }

        Ok(())
    }

    fn match_expression(&mut self) -> ParserResult<()> {
        println!("Expression");
        self.match_term()?;
        while self.is_current_plus_minus_token() {
            self.advance_token();
            self.match_term()?;
        }
        Ok(())
    }

    fn match_term(&mut self) -> ParserResult<()> {
        println!("Term");
        self.match_unary()?;
        while self.is_current_asterix_slash_token() {
            self.advance_token();
            self.match_unary()?;
        }
        Ok(())
    }
    fn match_unary(&mut self) -> ParserResult<()> {
        println!("Unary");
        if self.is_current_plus_minus_token() {
            self.advance_token();
        }
        self.match_primary()?;
        Ok(())
    }
    fn match_primary(&mut self) -> ParserResult<()> {
        println!("Primary");
        if self.is_current_token(TokenKind::Identifier) {
            let id = self.match_identifier()?;
            if !self.identifiers.contains(&id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is never declared", id)),
                });
            } else {
                println!("{}", id)
            }
        } else if self.is_current_literal_number() {
            //TODO capture Literal NR
            self.advance_token();
        } else {
            return Err(ParserError {
                token: self.current_token().cloned(),
                expected: Some(TokenKind::Identifier),
                reason: Some("Expected Identifier or literal nr".to_string()),
            });
        }
        Ok(())
    }

    fn match_comparison(&mut self) -> ParserResult<()> {
        println!("COMPARISON");

        self.match_expression()?;
        if self.is_current_comparison_token() {
            self.advance_token();
            self.match_expression()?;
        }

        while self.is_current_comparison_token() {
            self.advance_token();
            self.match_expression()?;
        }
        Ok(())
    }

    fn match_identifier(&mut self) -> ParserResult<String> {
        if self.is_current_token(TokenKind::Identifier) {
            println!("Identifier");
            let id = self.current_token().unwrap().data.raw.clone();
            self.advance_token();
            return Ok(id);
        }

        Err(ParserError {
            token: self.current_token().cloned(),
            expected: Some(TokenKind::Identifier),
            reason: Some("Expected Identifier".to_string()),
        })
    }

    fn match_token(&mut self, token_kind: TokenKind) -> ParserResult<()> {
        if self.is_current_token(token_kind.clone()) {
            println!("{:?}", token_kind);
            self.advance_token();
            Ok(())
        } else {
            Err(ParserError {
                token: self.current_token().cloned(),
                expected: Some(token_kind.clone()),
                reason: Some(format!("Expected {:?}", token_kind)),
            })
        }
    }

    fn is_current_asterix_slash_token(&self) -> bool {
        self.is_current_token(TokenKind::Asterisk) || self.is_current_token(TokenKind::Slash)
    }

    fn is_current_plus_minus_token(&self) -> bool {
        self.is_current_token(TokenKind::Plus) || self.is_current_token(TokenKind::Minus)
    }

    fn is_current_comparison_token(&self) -> bool {
        self.is_current_token(TokenKind::GreaterThan)
            || self.is_current_token(TokenKind::GreaterThanEquals)
            || self.is_current_token(TokenKind::LessThan)
            || self.is_current_token(TokenKind::LessThanEquals)
            || self.is_current_token(TokenKind::EqualsEquals)
            || self.is_current_token(TokenKind::NotEquals)
    }
    fn is_current_literal_number(&self) -> bool {
        match self.current_token() {
            Some(t) => match t.kind {
                TokenKind::LiteralNumber(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    fn is_current_token(&self, to_match_against: TokenKind) -> bool {
        match self.current_token() {
            Some(t) => t.kind == to_match_against,
            None => false,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance_token(&mut self) {
        self.current += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::lexer::TokenData;

    use super::*;

    #[macro_export]
    macro_rules! tokens {
    ($($kind:expr),* $(,)?) => {
        vec![
            $(
                Token {
                    kind: $kind,
                    data: TokenData { raw: "".into() },
                },
            )*
        ]
    };
}

    //TODO:
    //- Add support for passing Identifier name
    //- Test expression
    //- Test negative path
    #[test]
    fn check_if_basic_print_parses() {
        let tokens = tokens![
            TokenKind::Print,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_basic_multiline_parses() {
        let tokens = tokens![
            TokenKind::Print,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
            TokenKind::Whitespace,
            TokenKind::Print,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_white_space() {
        let tokens = tokens![
            TokenKind::Print,
            TokenKind::Whitespace,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_basic_if() {
        let tokens = tokens![
            TokenKind::If,
            TokenKind::Whitespace,
            TokenKind::LiteralNumber(3),
            TokenKind::GreaterThan,
            TokenKind::LiteralNumber(4),
            TokenKind::Whitespace,
            TokenKind::OpenCurly,
            TokenKind::Whitespace,
            TokenKind::Print,
            TokenKind::Whitespace,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
            TokenKind::Whitespace,
            TokenKind::CloseCurly,
            TokenKind::Whitespace
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_basic_while() {
        let tokens = tokens![
            TokenKind::While,
            TokenKind::Whitespace,
            TokenKind::LiteralNumber(3),
            TokenKind::GreaterThan,
            TokenKind::LiteralNumber(4),
            TokenKind::Whitespace,
            TokenKind::OpenCurly,
            TokenKind::Whitespace,
            TokenKind::Print,
            TokenKind::Whitespace,
            TokenKind::LiteralString,
            TokenKind::SemiColon,
            TokenKind::Whitespace,
            TokenKind::CloseCurly,
            TokenKind::Whitespace
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_basic_let() {
        let tokens = tokens![
            TokenKind::Let,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Equals,
            TokenKind::Whitespace,
            TokenKind::LiteralNumber(666),
            TokenKind::Whitespace,
            TokenKind::SemiColon,
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_basic_input() {
        let tokens = tokens![
            TokenKind::Input,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::SemiColon,
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }

    #[test]
    fn check_if_handles_basic_assignment() {
        let tokens = tokens![
            TokenKind::Let,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Equals,
            TokenKind::Whitespace,
            TokenKind::LiteralNumber(666),
            TokenKind::Whitespace,
            TokenKind::SemiColon,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Equals,
            TokenKind::Whitespace,
            TokenKind::LiteralNumber(664),
            TokenKind::SemiColon
        ];
        let mut parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }
}
