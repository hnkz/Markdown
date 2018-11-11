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

        let mut parser = Parser::new(tokens);
        let obj = parser.parse();
        
        Markdown {
            obj: obj
        }
    }
}

impl MDObject for Markdown {
    fn output(&self) {
        for obj in &self.obj {
            obj.output();
        }
    }
}