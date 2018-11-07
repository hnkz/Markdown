 
use super::super::object::{
    mdobject::MDObject,
    head1,
    head2,
    head3,
    head4,
    head5,
    list,
    paragragh::Paragragh,
};

#[derive(Debug)]
pub enum ParseState {
    None,
    Normal,
    Head1, // #
    Head2, // ##
    Head3, // ###
    Head4, // ####
    Head5, // #####
    List, // -, +, *
    Order, // 1.
    CodeInline, // `
    CodeBlock, // ```
    TableHead, // |--|--|
    TableContents,
    Link, // [test](url)
    Image, // ![alt text][url]
    Quote, // >
    Bold, // __text__ , **text**
    Italic, // _text_, *text*
    Strikethrough,// ~~text~~
    Line, // ---, ***, +++
}

impl ParseState {
    pub fn get_obj(state: ParseState) -> Box<MDObject> {
        match state {
            // None,
            Paragragh => Box::new(Paragragh::new()),
            // Head1 => Head1::new(),
            // Head2,
            // Head3,
            // Head4,
            // Head5,
            // List,
            // Order,
            // CodeInline,
            // CodeBlock,
            // Table,
            // Link,
            // Image,
            // Quote,
            // Bold,
            // Italic,
            // Strikethrough,
            // Line,
            _ => Box::new(Paragragh::new())
        }
    }
}