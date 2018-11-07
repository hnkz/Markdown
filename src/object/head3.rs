use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head3 {
    val: String,
}

impl Head3 {
    pub fn new() -> Head3 {
        Head3 {
            val: String::new(),
        }
    }
}

impl MDObject for Head3 {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}