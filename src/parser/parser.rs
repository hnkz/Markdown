use super::Token;
use super::TokenType;
use super::ParseState;

#[derive(Debug)]
pub struct Parser {
    state: ParseState,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParseState::None,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> () {
        for token in tokens {
            match self.state {
                ParseState::None => {
                    match token.get_state() {
                        TokenType::Head1 => {

                        }
                        _ => {

                        }
                    }
                }
                _ => {}
            }
        }
    }
}