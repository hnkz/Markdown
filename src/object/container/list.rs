use super::MDObject;
use super::Str;

#[derive(Debug)]
pub struct List {
    val: Str,
}

impl List {
    pub fn new(val: Str) -> List {
        List {
            val: val,
        }
    }
}

impl MDObject for List {
    fn output(&self) {
        println!("{:?}", self);
    }
}