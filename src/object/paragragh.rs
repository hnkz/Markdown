use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Paragragh {
    val: String,
}

impl Paragragh {
    pub fn new() -> Paragragh {
        Paragragh {
            val: String::new(),
        }
    }
}

impl MDObject for Paragragh {
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}