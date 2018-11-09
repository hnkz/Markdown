
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

}