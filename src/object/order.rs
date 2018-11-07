use super::mdobject::MDObject;

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
    fn parse(&mut self, buf: &mut str) {
        self.val = buf.to_string();
    }
}