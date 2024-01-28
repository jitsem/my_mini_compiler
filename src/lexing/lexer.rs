use std::collections::HashMap;

pub type LexerResult<T> = std::result::Result<T, LexerError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct LexerError;

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    keywords: HashMap<String, TokenKind>,
}

pub struct Token {
    pub kind: TokenKind,
    pub data: TokenData,
}

pub struct TokenData {
    pub raw: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
    LiteralNumber(i64),
    LiteralString,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Let,
    If,
    Else,
    While,
    OpenCurly,
    CloseCurly,
    SemiColon,
    Print,
    Input,
    Invalid,
    Whitespace,
    Identifier,
    Eof,
    TokenizationError,
}

impl Token {
    fn new(kind: TokenKind, data: String) -> Self {
        Token {
            kind,
            data: TokenData { raw: data },
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(t) if t.kind == TokenKind::Eof => None,
            Ok(t) => Some(t),
            Err(_) => Some(Token::new(TokenKind::TokenizationError, "".into())),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn from(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            current_pos: 0,
            keywords: HashMap::new(),
        };
        lexer.keywords.insert("let".to_string(), TokenKind::Let);
        lexer.keywords.insert("if".to_string(), TokenKind::If);
        lexer.keywords.insert("else".to_string(), TokenKind::Else);
        lexer.keywords.insert("while".to_string(), TokenKind::While);
        lexer.keywords.insert("print".to_string(), TokenKind::Print);
        lexer.keywords.insert("input".to_string(), TokenKind::Input);
        lexer
    }

    pub fn next_token(&mut self) -> LexerResult<Token> {
        let token = match self.current_char() {
            None => Token::new(TokenKind::Eof, '\0'.into()),
            Some(c) if c.is_ascii_digit() => {
                let mut str = String::new();
                str.push(c);
                loop {
                    let next = self.peek_next_char();
                    match next {
                        Some(c) if c.is_ascii_digit() => {
                            str.push(c);
                            self.next_char();
                        }
                        _ => {
                            let int: i64 = str.parse::<i64>().map_err(|_| LexerError)?;
                            break Token::new(TokenKind::LiteralNumber(int), str);
                        }
                    };
                }
            }
            Some(c) if c.is_ascii_alphabetic() => {
                //We always start with a alphabet leter
                let mut str = String::new();
                str.push(c);
                loop {
                    let next = self.peek_next_char();
                    match next {
                        Some(c) if c.is_ascii_alphanumeric() => {
                            str.push(c);
                            self.next_char();
                        }
                        _ => {
                            break match self.keywords.get(&str) {
                                Some(v) => Token::new(*v, str),
                                _ => Token::new(TokenKind::Identifier, str),
                            }
                        }
                    };
                }
            }
            Some(c) => match c {
                '\n' => Token::new(TokenKind::Whitespace, c.into()),
                '\r' => Token::new(TokenKind::Whitespace, c.into()),
                '\t' => Token::new(TokenKind::Whitespace, c.into()),
                ' ' => Token::new(TokenKind::Whitespace, c.into()),
                '+' => Token::new(TokenKind::Plus, c.into()),
                '-' => Token::new(TokenKind::Minus, c.into()),
                '*' => Token::new(TokenKind::Asterisk, c.into()),
                '/' => Token::new(TokenKind::Slash, c.into()),
                '{' => Token::new(TokenKind::OpenCurly, c.into()),
                '}' => Token::new(TokenKind::CloseCurly, c.into()),
                ';' => Token::new(TokenKind::SemiColon, c.into()),
                '<' => match self.peek_next_char() {
                    Some(second) if second == '=' => {
                        self.next_char().ok_or(LexerError)?;
                        Token::new(TokenKind::LessThanEquals, format!("{}=", c))
                    }
                    _ => Token::new(TokenKind::LessThan, c.into()),
                },
                '>' => match self.peek_next_char() {
                    Some(second) if second == '=' => {
                        self.next_char().ok_or(LexerError)?;
                        Token::new(TokenKind::GreaterThanEquals, format!("{}=", c))
                    }
                    _ => Token::new(TokenKind::GreaterThan, c.into()),
                },
                '=' => match self.peek_next_char() {
                    Some(second) if second == '=' => {
                        self.next_char().ok_or(LexerError)?;
                        Token::new(TokenKind::EqualsEquals, format!("{}=", c))
                    }
                    _ => Token::new(TokenKind::Equals, c.into()),
                },
                '!' => match self.peek_next_char() {
                    Some(second) if second == '=' => {
                        self.next_char().ok_or(LexerError)?;
                        Token::new(TokenKind::NotEquals, format!("{}=", c))
                    }
                    _ => Token::new(TokenKind::Invalid, c.into()),
                },
                '"' => {
                    let mut str = String::new();
                    str.push(c);
                    loop {
                        let next = self.next_char();
                        match next {
                            None => break Token::new(TokenKind::Invalid, str),
                            Some(c) => {
                                str.push(c);
                                match c {
                                    '\"' => break Token::new(TokenKind::LiteralString, str),
                                    '\n' => break Token::new(TokenKind::Invalid, str),
                                    '\r' => break Token::new(TokenKind::Invalid, str),
                                    _ => {}
                                }
                            }
                        };
                    }
                }

                _ => Token::new(TokenKind::Invalid, c.into()),
            },
        };
        self.next_char();
        Ok(token)
    }

    fn current_char(&self) -> Option<char> {
        if self.is_done() {
            return None;
        }
        self.input.chars().nth(self.current_pos)
    }
    fn next_char(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.current_char()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    fn is_done(&self) -> bool {
        self.current_pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenizing() {
        let mut lexer = Lexer::from("+-*/=={}<>;!=<==>=! \n\r\t");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Plus);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Minus);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Asterisk);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Slash);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::EqualsEquals);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::OpenCurly);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::CloseCurly);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LessThan);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::GreaterThan);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::SemiColon);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::NotEquals);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LessThanEquals);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Equals);
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::GreaterThanEquals
        );
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Invalid);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_basic_string() {
        let mut lexer = Lexer::from("\"Hello World\"");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LiteralString);
        assert_eq!(token.data.raw, "\"Hello World\"");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_multiline_string_not_allowed() {
        let mut lexer = Lexer::from("\"Hell\nWorld\"");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Invalid);
        assert_eq!(token.data.raw, "\"Hell\n");
    }

    #[test]
    fn test_unclose_string_not_allowed() {
        let mut lexer = Lexer::from("\"Hello World");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Invalid);
        assert_eq!(token.data.raw, "\"Hello World");
    }

    #[test]
    fn test_basic_nr() {
        let mut lexer = Lexer::from("1");
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::LiteralNumber(1)
        );
    }

    #[test]
    fn test_big_nr() {
        let mut lexer = Lexer::from("987654321");
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::LiteralNumber(987654321)
        );
    }

    #[test]
    fn test_two_nr() {
        let mut lexer = Lexer::from("9876 4321");
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::LiteralNumber(9876)
        );
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(
            lexer.next_token().unwrap().kind,
            TokenKind::LiteralNumber(4321)
        );
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::from("let if else while while= whileif hello\nprint;input");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Let);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::If);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Else);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::While);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::While);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Equals);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Identifier);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Identifier);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Print);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::SemiColon);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Input);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::from("Hello World");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.data.raw, "Hello");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Whitespace);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.data.raw, "World");
    }

    #[test]
    fn test_interator() {
        let lexer = Lexer::from("let if else while while= whileif hello\nprint;input");
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap().kind, TokenKind::Let);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::If);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Else);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::While);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::While);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Equals);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Whitespace);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Print);
        assert_eq!(iter.next().unwrap().kind, TokenKind::SemiColon);
        assert_eq!(iter.next().unwrap().kind, TokenKind::Input);
        assert_eq!(iter.next().is_none(), true);
    }
}
