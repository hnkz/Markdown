
use super::TokenType;

#[derive(Debug, Clone)]
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

    pub fn get_state(&self) -> TokenType {
        self.t_type.clone()
    }

    pub fn get_val(&self) -> String {
        self.val.to_owned()
    }
}