use crate::lexer::tokens::Token;

pub fn make_html(tokens: Vec<Token>){
    let mut html = String::new();
    for token in tokens{
        let string = match token{
            Token::Paragraph(text) => format!("<p>{}</p>", text),
            Token::Heading(level, text) => format!("<h{}>{}</h{}>", level, text, level),
            Token::Bold(text) => format!("<b>{}</b>", text),
            Token::Italic(text) => format!("<i>{}</i>", text),
            _ => String::from("Not Implemented"),
        };
        html.push_str(string.as_str());
    }
    println!("{}", html);
}