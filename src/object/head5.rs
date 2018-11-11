use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head5 {
    val: String,
}

impl Head5 {
    pub fn new(val: String) -> Head5 {
        Head5 {
            val: val,
        }
    }
}

impl MDObject for Head5 {
    fn output(&self) {
        println!("{:?}", self);
    }
}