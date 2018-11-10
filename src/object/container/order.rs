use super::MDObject;

#[derive(Debug)]
pub struct Order {
    num: i32,
    val: String,
}

impl Order {
    pub fn new(num: i32) -> Order {
        Order {
            num: num,
            val: String::new(),
        }
    }
}

impl MDObject for Order {
    fn output(&self) {
        
    }
}