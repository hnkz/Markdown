use super::mdobject::MDObject;

#[derive(Debug)]
pub struct List {
    val: String,
}

impl List {
    pub fn new() -> List {
        List {
            val: String::new(),
        }
    }
}

impl MDObject for List {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}