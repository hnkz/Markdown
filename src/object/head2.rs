use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head2 {
    val: String,
}

impl Head2 {
    pub fn new(val: String) -> Head2 {
        Head2 {
            val: val,
        }
    }
}

impl MDObject for Head2 {
    fn output(&self) {
        println!("{:?}", self);
    }
}