use super::MDObject;
use super::Str;

#[derive(Debug)]
pub struct Order {
    num: i32,
    val: Str,
}

impl Order {
    pub fn new(num: i32, val: Str) -> Order {
        Order {
            num: num,
            val: val,
        }
    }
}

impl MDObject for Order {
    fn output(&self) {
        
    }
}