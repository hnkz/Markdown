use super::MDObject;
use super::parser::Parser;
use super::tokenizer::{ Tokenizer, Token };

pub struct Markdown {
    obj: Vec<Box<MDObject>>,
}

impl Markdown {
    pub fn new(buf: &mut str) -> Markdown {
        let mut tokenizer = Tokenizer::new(buf);
        let tokens: Vec<Token> = tokenizer.tokenize();

        let mut parser = Parser::new();
        let obj = parser.parse(tokens);
        
        Markdown {
            obj: obj
        }
    }
}

impl MDObject for Markdown {
    fn output(&self) {
        println!("{}", self.obj.len());
        for obj in &self.obj {
            obj.output();
        }
    }
}