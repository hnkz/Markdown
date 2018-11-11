use super::StrType;
use std::ops::Add;

#[derive(Debug)]
pub struct Str {
    val: Vec<StrType>
}

impl Str {
    pub fn new() -> Self {
        Str {
            val: Vec::new(),
        }
    }

    pub fn push(&mut self, val: StrType) {
        self.val.push(val);
    }

    pub fn add_vec(&mut self, other: Str) {
        for val in other.val {
            self.val.push(val);
        }
    }
}