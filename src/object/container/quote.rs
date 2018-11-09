use super::MDObject;

#[derive(Debug)]
pub struct Quote {
    val: String,
}

impl Quote {
    pub fn new() -> Quote {
        Quote {
            val: String::new(),
        }
    }
}

impl MDObject for Quote {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}