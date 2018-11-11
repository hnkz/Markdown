use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head3 {
    val: String,
}

impl Head3 {
    pub fn new(val: String) -> Head3 {
        Head3 {
            val: val,
        }
    }
}

impl MDObject for Head3 {
    fn output(&self) {
        println!("{:?}", self);
    }
}