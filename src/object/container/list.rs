use super::MDObject;

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
    fn output(&self) {
        
    }
}