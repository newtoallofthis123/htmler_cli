
#[derive(Debug, PartialEq)]
pub enum Token{
    BOF,
    Heading(u8, String),
    Paragraph(String),
    Bold(String),
    Italic(String),
    Code(String),
    CodeBlock(String),
    BlockQuote(String), 
    HorizontalRule,
    ListItem(String),
    Link(String, String),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Image(String, String),
    NewLine,
    Skipped,
    EOF,
}