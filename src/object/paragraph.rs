use super::mdobject::MDObject;
use super::string::Str;

#[derive(Debug)]
pub struct Paragraph {
    val: Str,
}

impl Paragraph {
    pub fn new(val: Str) -> Paragraph {
        Paragraph {
            val: val,
        }
    }
}

impl MDObject for Paragraph {
    fn output(&self) {
        println!("{:#?}", self);
    }
}