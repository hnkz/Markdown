use super::mdobject::MDObject;

#[derive(Debug)]
pub struct Head4 {
    val: String,
}

impl Head4 {
    pub fn new(val: &mut str) -> Head4 {
        Head4 {
            val: val.to_string(),
        }
    }
}

impl MDObject for Head4 {
    fn output(&self) {
        println!("{:?}", self);
    }
}