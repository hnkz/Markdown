use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head1 {
    val: String,
}

impl Head1 {
    pub fn new(val: String) -> Head1 {
        Head1 {
            val: val,
        }
    }
}

impl MDObject for Head1 {
    fn output(&self) {
        println!("{:?}", self);
    }
}