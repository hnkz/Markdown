use super::Container;

#[derive(Debug)]
pub enum Inner<T> {
    Container(Box<Container<T>>),
    Val(T)
}