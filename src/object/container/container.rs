use super::MDObject;
use super::List;
use super::Order;
use super::Quote;

#[derive(Debug)]
pub struct Container<T> {
    inner: Inner<T>
}

// impl<T> Container<T> {
//     pub fn newlist() -> Container<List> {
//         Container<List> {
//             inner: Inner<List>::None,
//         }
//     }

//     pub fn neworder() -> Container<Order> {

//     }

//     pub fn newquote() -> Container<Quote> {

//     }
// }

#[derive(Debug)]
enum Inner<T> {
    Container(Box<Container<T>>),
    Val(Vec<T>),
    None
}