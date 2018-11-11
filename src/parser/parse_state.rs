
#[derive(Debug)]
pub enum ParseState {
    None,
    Head1, // #
    Head2, // ##
    Head3, // ###
    Head4, // ####
    Head5, // #####
    List, // -, +, *
    Order, // 1.
    CodeInline, // `
    CodeBlock, // ```
    Table,
    Link, // [test](url)
    Image, // ![alt text][url]
    Quote, // >
    Line, // ---, ***, +++
    Paragraph,
}

impl ParseState {

}