use super::MDObject;

#[derive(Debug)]
pub struct Quote {
    val: String,
}

impl Quote {
    pub fn new(val: String) -> Quote {
        Quote {
            val: val,
        }
    }
}

impl MDObject for Quote {
    fn output(&self) {
        println!("{:?}", self);
    }
}