
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Head1, // "# "
    Head2, // "## "
    Head3, // "### "
    Head4, // "#### "
    Head5, // "##### "
    List, // "- "
    Order, // "1." .. "9."
    CodeInline, // "`"
    CodeBlock, // "```"
    TableStart,
    TableColumn,
    TableContents,
    TableEnd,
    URLLinkStart, // "["
    LinkEnd, // "]"
    URLStart, // "("
    URLEnd, // ")"
    ImageLinkStart, // "!["
    Quote, // ">"
    Bold, // **"
    Italic, // "*"
    Strikethrough, // "~~"
    Line, // "---"
    Str,
    NewLine, // "\n"
    Space, // " "
    Tab, // "\t"
    None,
}