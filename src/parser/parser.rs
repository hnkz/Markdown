use super::Token;
use super::TokenType;
use super::ParseState;
use super::object::MDObject;
use super::object::{
    Head1, Head2, Head3, Head4, Head5,
};

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

    pub fn parse(&mut self, tokens: Vec<Token>) -> Vec<Box<MDObject>> {
        let mut objs: Vec<Box<MDObject>> = Vec::new();
        let mut val = String::new();
        for token in tokens {
            match self.state {
                ParseState::None => {
                    match token.get_state() {
                        TokenType::Head1 => {
                            self.state = ParseState::Head1;
                        }
                        TokenType::Head2 => {
                            self.state = ParseState::Head2;
                        }
                        TokenType::Head3 => {
                            self.state = ParseState::Head3;
                        }
                        TokenType::Head4 => {
                            self.state = ParseState::Head4;
                        }
                        TokenType::Head5 => {
                            self.state = ParseState::Head5;
                        }
                        _ => {

                        }
                    }
                }
                ParseState::Head1 => {
                    match token.get_state() {
                        TokenType::Str => {
                            
                        }
                        TokenType::NewLine => {
                            let mut val = "test h1";
                            let obj = Head1::new(val);
                            objs.push(Box::new(obj));
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        objs
    }
}