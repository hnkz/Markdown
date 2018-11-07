use super::token::Token;

#[derive(Debug)]
pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {

        }
    }

    pub fn tokenize(&mut self, buf: &mut str) -> Vec<Token> {

        Vec::new()
    }
}