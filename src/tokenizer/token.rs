
use super::TokenType;

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    val: String,
}

impl Token {
    pub fn new(val: String, t_type: TokenType) -> Token {
        Token {
            t_type: t_type,
            val: val,
        }
    }
}