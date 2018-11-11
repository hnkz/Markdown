
#[derive(Debug)]
pub enum StrType {
    Normal(String),
    Link((String, String)),
    Italic(String),
    Bold(String),
    CodeInline(String),
    Strikethrough(String),
}