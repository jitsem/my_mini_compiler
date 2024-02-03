use core::panic;
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

#[derive(Debug, Clone)]
pub enum Statement {
    Print {
        option: PrintOption,
    },
    If {
        comparison: Comparison,
        statements: Vec<Statement>,
    },

    While {
        comparison: Comparison,
        statements: Vec<Statement>,
    },

    Let {
        identifier: Identifier,
        expression: Expression,
    },
    Input {
        identifier: Identifier,
    },
    Assign {
        identifier: Identifier,
        expression: Expression,
    },
}
#[derive(Debug, Clone)]
pub enum Comparison {
    GreaterThan { lhs: Expression, rhs: Expression },
    GreaterThanEquals { lhs: Expression, rhs: Expression },
    LessThan { lhs: Expression, rhs: Expression },
    LessThanEquals { lhs: Expression, rhs: Expression },
    EqualsEquals { lhs: Expression, rhs: Expression },
    NotEquals { lhs: Expression, rhs: Expression },
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub lhs: Term,
    pub rhs: Box<Option<ExpressionOp>>,
}

#[derive(Debug, Clone)]
pub enum ExpressionOp {
    Plus(Expression),
    Minus(Expression),
}

#[derive(Debug, Clone)]
pub struct Term {
    pub lhs: Unary,
    pub rhs: Box<Option<TermOp>>,
}

#[derive(Debug, Clone)]
pub enum TermOp {
    Multiply(Term),
    Divide(Term),
}

#[derive(Debug, Clone)]
pub enum Unary {
    Positive(Primary),
    Negative(Primary),
    UnSigned(Primary),
}

#[derive(Debug, Clone)]
pub enum Primary {
    LiteralNumber(i64),
    IdentifierExpression(Identifier),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum PrintOption {
    PrintLiteral(String),
    PrintExpression(Expression),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    identifiers: HashSet<String>,
    statements: Vec<Statement>,
}

impl Parser {
    //Todo, allow less explicit token input
    pub fn new(tokens: &[Token]) -> Self {
        let tokens: Vec<Token> = tokens
            .iter()
            .filter(|t| t.kind != TokenKind::Whitespace)
            .cloned()
            .collect();
        let identifiers: HashSet<String> = HashSet::new();
        let statements: Vec<Statement> = Vec::new();
        Self {
            tokens,
            current: 0,
            identifiers,
            statements,
        }
    }

    pub fn parse(mut self) -> ParserResult<Vec<Statement>> {
        while let Some(token) = self.current_token() {
            match token.kind {
                TokenKind::Eof => break,
                _ => {
                    let statement = self.match_statement()?;
                    self.statements.push(statement)
                }
            }
        }

        Ok(self.statements)
    }

    fn match_statement(&mut self) -> ParserResult<Statement> {
        if self.is_current_token(TokenKind::Print) {
            self.advance_token();
            let option = {
                if self.is_current_token(TokenKind::LiteralString) {
                    let literal: String = self
                        .current_token()
                        .expect("Expected current token")
                        .data
                        .raw
                        .replace('\"', "") //TODO could be cleaner
                        .to_string();
                    self.advance_token();
                    PrintOption::PrintLiteral(literal)
                } else {
                    let expression = self.match_expression()?;
                    PrintOption::PrintExpression(expression)
                }
            };
            self.match_token(TokenKind::SemiColon)?;
            Ok(Statement::Print { option })
        } else if self.is_current_token(TokenKind::If) {
            self.advance_token();
            let mut statements = Vec::new();
            let comparison = self.match_comparison()?;
            self.match_token(TokenKind::OpenCurly)?;
            while !self.is_current_token(TokenKind::CloseCurly) {
                let statement = self.match_statement()?;
                statements.push(statement);
            }
            self.match_token(TokenKind::CloseCurly)?;

            return Ok(Statement::If {
                comparison,
                statements,
            });
        } else if self.is_current_token(TokenKind::While) {
            self.advance_token();

            let mut statements = Vec::new();
            let comparison = self.match_comparison()?;
            self.match_token(TokenKind::OpenCurly)?;
            while !self.is_current_token(TokenKind::CloseCurly) {
                let statement = self.match_statement()?;
                statements.push(statement);
            }
            self.match_token(TokenKind::CloseCurly)?;

            return Ok(Statement::While {
                comparison,
                statements,
            });
        } else if self.is_current_token(TokenKind::Let) {
            self.advance_token();
            let identifier = self.match_identifier()?;
            if self.identifiers.contains(&identifier.id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is already declared", identifier.id)),
                });
            } else {
                self.identifiers.insert(identifier.id.clone());
            }
            self.match_token(TokenKind::Equals)?;
            let expression = self.match_expression()?;
            self.match_token(TokenKind::SemiColon)?;
            Ok(Statement::Let {
                identifier,
                expression,
            })
        } else if self.is_current_token(TokenKind::Input) {
            self.advance_token();
            let identifier = self.match_identifier()?;
            if self.identifiers.contains(&identifier.id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is already declared", identifier.id)),
                });
            } else {
                self.identifiers.insert(identifier.id.clone());
            }
            self.match_token(TokenKind::SemiColon)?;
            Ok(Statement::Input { identifier })
        } else if self.is_current_token(TokenKind::Identifier) {
            let identifier = self.match_identifier()?;
            if !self.identifiers.contains(&identifier.id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is never declared", identifier.id)),
                });
            }
            self.match_token(TokenKind::Equals)?;
            let expression = self.match_expression()?;
            self.match_token(TokenKind::SemiColon)?;
            Ok(Statement::Assign {
                identifier,
                expression,
            })
        } else {
            return Err(ParserError {
                token: self.current_token().cloned(),
                expected: None,
                reason: Some("Unknown statement".to_string()),
            });
        }
    }

    fn match_expression(&mut self) -> ParserResult<Expression> {
        let lhs = self.match_term()?;
        let token = {
            if self.is_current_plus_minus_token() {
                Some(self.match_plus_minus_token()?)
            } else {
                None
            }
        };
        let rhs = match token {
            Some(TokenKind::Plus) => {
                let exp = self.match_expression()?;
                Some(ExpressionOp::Plus(exp))
            }
            Some(TokenKind::Minus) => {
                let exp = self.match_expression()?;
                Some(ExpressionOp::Minus(exp))
            }
            Some(_) => panic!("Should not come here"),
            None => None,
        };
        Ok(Expression {
            lhs,
            rhs: Box::new(rhs),
        })
    }

    fn match_term(&mut self) -> ParserResult<Term> {
        let lhs = self.match_unary()?;
        let token = {
            if self.is_current_asterix_slash_token() {
                Some(self.match_asterix_slash_token()?)
            } else {
                None
            }
        };
        let rhs = match token {
            Some(TokenKind::Asterisk) => {
                let term = self.match_term()?;
                Some(TermOp::Multiply(term))
            }
            Some(TokenKind::Slash) => {
                let term = self.match_term()?;
                Some(TermOp::Divide(term))
            }
            Some(_) => panic!("Should not come here"),
            None => None,
        };
        Ok(Term {
            lhs,
            rhs: Box::new(rhs),
        })
    }
    fn match_unary(&mut self) -> ParserResult<Unary> {
        let token = {
            if self.is_current_plus_minus_token() {
                Some(self.match_plus_minus_token()?)
            } else {
                None
            }
        };
        let primary = self.match_primary()?;
        match token {
            Some(TokenKind::Plus) => Ok(Unary::Positive(primary)),
            Some(TokenKind::Minus) => Ok(Unary::Negative(primary)),
            Some(_) => panic!("Should not come here"),
            None => Ok(Unary::UnSigned(primary)),
        }
    }
    fn match_primary(&mut self) -> ParserResult<Primary> {
        if self.is_current_token(TokenKind::Identifier) {
            let identifier = self.match_identifier()?;
            if !self.identifiers.contains(&identifier.id) {
                return Err(ParserError {
                    token: self.current_token().cloned(),
                    expected: None,
                    reason: Some(format!("Identifier {} is never declared", &identifier.id)),
                });
            } else {
                Ok(Primary::IdentifierExpression(identifier))
            }
        } else if self.is_current_literal_number() {
            if let TokenKind::LiteralNumber(nr) =
                self.current_token().expect("Expected current token").kind
            {
                self.advance_token();
                return Ok(Primary::LiteralNumber(nr));
            } else {
                panic!("Should not come here");
            }
        } else {
            return Err(ParserError {
                token: self.current_token().cloned(),
                expected: Some(TokenKind::Identifier),
                reason: Some("Expected Identifier or literal nr".to_string()),
            });
        }
    }

    fn match_comparison(&mut self) -> ParserResult<Comparison> {
        let lhs = self.match_expression()?;
        if self.is_current_comparison_token() {
            let token = self.match_comparison_token()?;
            let rhs = self.match_expression()?;
            match token {
                TokenKind::GreaterThan => Ok(Comparison::GreaterThan { lhs, rhs }),
                TokenKind::GreaterThanEquals => Ok(Comparison::GreaterThanEquals { lhs, rhs }),
                TokenKind::LessThan => Ok(Comparison::LessThan { lhs, rhs }),
                TokenKind::LessThanEquals => Ok(Comparison::LessThanEquals { lhs, rhs }),
                TokenKind::EqualsEquals => Ok(Comparison::EqualsEquals { lhs, rhs }),
                TokenKind::NotEquals => Ok(Comparison::NotEquals { lhs, rhs }),
                _ => panic!("Should not come here"),
            }
        } else {
            return Err(ParserError {
                token: self.current_token().cloned(),
                expected: None,
                reason: Some("Expected comparison operator".to_string()),
            });
        }

        // TODO comparisons can be chained.
        // while self.is_current_comparison_token() {
        //     self.advance_token();
        //     self.match_expression()?;
        // }
    }

    fn match_identifier(&mut self) -> ParserResult<Identifier> {
        if self.is_current_token(TokenKind::Identifier) {
            let id = self.current_token().unwrap().data.raw.clone();
            self.advance_token();
            return Ok(Identifier { id });
        }

        Err(ParserError {
            token: self.current_token().cloned(),
            expected: Some(TokenKind::Identifier),
            reason: Some("Expected Identifier".to_string()),
        })
    }

    fn match_token(&mut self, token_kind: TokenKind) -> ParserResult<()> {
        if self.is_current_token(token_kind) {
            self.advance_token();
            Ok(())
        } else {
            Err(ParserError {
                token: self.current_token().cloned(),
                expected: Some(token_kind),
                reason: Some(format!("Expected {:?}", token_kind)),
            })
        }
    }

    fn match_comparison_token(&mut self) -> ParserResult<TokenKind> {
        if self.is_current_comparison_token() {
            let kind = self.current_token().unwrap().kind;
            self.advance_token();
            Ok(kind)
        } else {
            Err(ParserError {
                token: None,
                expected: None,
                reason: Some("Expected comparison token".to_string()),
            })
        }
    }

    fn match_plus_minus_token(&mut self) -> ParserResult<TokenKind> {
        if self.is_current_plus_minus_token() {
            let kind = self.current_token().unwrap().kind;
            self.advance_token();
            Ok(kind)
        } else {
            Err(ParserError {
                token: None,
                expected: None,
                reason: Some("Expected signed token".to_string()),
            })
        }
    }

    fn match_asterix_slash_token(&mut self) -> ParserResult<TokenKind> {
        if self.is_current_asterix_slash_token() {
            let kind = self.current_token().unwrap().kind;
            self.advance_token();
            Ok(kind)
        } else {
            Err(ParserError {
                token: None,
                expected: None,
                reason: Some("Expected * or / token".to_string()),
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
            Some(t) => matches!(t.kind, TokenKind::LiteralNumber(_)),
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
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
        let parser = Parser::new(&tokens);
        let res = parser.parse();
        assert!(res.is_ok(), "Test failed with error: {:?}", res);
    }
}
