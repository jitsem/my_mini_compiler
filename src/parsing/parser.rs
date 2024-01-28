use crate::lexing::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
}

impl Parser {
    //Todo, allow less explicit token input
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: 0,
        }
    }

    pub fn parse() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_if_works() {
        assert!(true)
    }
}
