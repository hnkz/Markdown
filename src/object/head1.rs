use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head1 {
    val: String,
}

impl Head1 {
    pub fn new(val: &mut str) -> Head1 {
        Head1 {
            val: val.to_string(),
        }
    }
}

impl MDObject for Head1 {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}