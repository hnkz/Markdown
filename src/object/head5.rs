use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head5 {
    val: String,
}

impl Head5 {
    pub fn new(val: &mut str) -> Head5 {
        Head5 {
            val: val.to_string(),
        }
    }
}

impl MDObject for Head5 {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}