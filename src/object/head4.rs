use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head4 {
    val: String,
}

impl Head4 {
    pub fn new() -> Head4 {
        Head4 {
            val: String::new(),
        }
    }
}

impl MDObject for Head4 {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}