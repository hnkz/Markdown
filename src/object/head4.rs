use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head4 {
    val: String,
}

impl Head4 {
    pub fn new(val: String) -> Head4 {
        Head4 {
            val: val,
        }
    }
}

impl MDObject for Head4 {
    fn output(&self) {
        println!("{:?}", self);
    }
}