use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head2 {
    val: String,
}

impl Head2 {
    pub fn new(val: &mut str) -> Head2 {
        Head2 {
            val: val.to_string(),
        }
    }
}

impl MDObject for Head2 {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}