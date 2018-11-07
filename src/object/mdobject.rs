pub trait MDObject {
    fn parse(&mut self, buf: &mut str)
        where Self : Sized;
}