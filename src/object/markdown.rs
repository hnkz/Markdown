use super::{
    MDObject,
    Head1,
    Head2,
    Head3,
    Head4,
    Head5,
    List,
    Order,
    Paragragh,
    Line
};

use super::parser::Parser;
use super::tokenizer::{ Tokenizer, Token };

pub struct Markdown {
    obj: Vec<Box<MDObject>>,
}

impl Markdown {
    pub fn new(buf: &mut str) -> Markdown {
        let mut tokenizer = Tokenizer::new();
        let tokens: Vec<Token> = tokenizer.tokenize(buf);
        let mut parser = Parser::new();
        let obj = parser.parse(tokens);
        
        Markdown {
            obj: Vec::new()
        }
    }
}