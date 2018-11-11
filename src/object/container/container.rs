use super::MDObject;
use super::List;
use super::Order;
use super::Quote;
use super::Inner;

#[derive(Debug)]
pub struct Container<T> {
    inner: Vec<Inner<T>>
}

impl<T> Container<T> {
    pub fn new<IN>() -> Container<IN> {
        Container {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, val: Inner<T>) {
        self.inner.push(val);
    }

    pub fn push_vec(&mut self, vals: Vec<Inner<T>>) {
        for val in vals {
            self.inner.push(val);
        }
    }
}

impl MDObject for Container<List> {
    fn output(&self) {
        println!("{:#?}", self);
    }
}

impl MDObject for Container<Order> {
    fn output(&self) {
        println!("{:#?}", self);
    }
}

impl MDObject for Container<Quote> {
    fn output(&self) {
        println!("{:#?}", self);
    }
}