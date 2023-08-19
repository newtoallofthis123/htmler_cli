pub mod tokens;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn peek_char(&self) -> Option<char> {
        self.current_char
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    fn count_heading_level(&mut self)->u8{
        let mut heading_level = 1;
        while self.peek_char() == Some('#') {
            self.advance();
            heading_level += 1;
        }
        heading_level
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == ' '{
                self.advance();
            } else {
                break;
            }
        }
    }

    fn extract_heading_text(&mut self) -> String {
        let mut heading_text = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            heading_text.push(ch);
            self.advance();
        }
        heading_text.trim().to_string()
    }

    pub fn tokenize(&mut self) -> Option<tokens::Token> {
        match self.peek_char() {
            // empty
            None => None,
            Some('#') => {
                self.advance();

                let heading_level = self.count_heading_level();
                while self.peek_char() == Some(' ') {
                    self.advance();
                }
                let heading_text = self.extract_heading_text();
                if heading_text.trim().is_empty() {
                    return Some(tokens::Token::Skipped);
                }
                self.advance();
                return Some(tokens::Token::Heading(heading_level, heading_text.trim().to_string()));
            }
            Some('`') => {
                let mut count: u8 = 0;
                while self.peek_char() == Some('`'){
                    count += 1;
                    self.advance();
                }
                let mut code_text = String::new();
                while let Some(ch) = self.peek_char() {
                    if count == 1 && ch == '`' {
                        self.advance();
                        break;
                    } else if count == 3 && ch == '`' {
                        self.advance();
                        self.advance();
                        break;
                    }
                    code_text.push(ch);
                    self.advance();
                    if count == 3 && self.peek_char() == Some('\n') {
                        self.advance();
                        break;
                    }
                }
                if count == 1{
                        return Some(tokens::Token::Code(code_text.trim().to_string()));
                }
                else {
                    return Some(tokens::Token::CodeBlock(code_text.trim().to_string()));
                }
            }
            Some('_') => {
                let mut count: u8 = 0;
                while self.peek_char() == Some('_') {
                    count += 1;
                    self.advance();
                }
                let mut text = String::new();
                while let Some(ch) = self.peek_char() {
                    if count == 1 && ch == '_' {
                        self.advance();
                        break;
                    } else if count == 2 && ch == '_' {
                        break;
                    }
                    text.push(ch);
                    self.advance();
                }
                if text.trim().is_empty() {
                    return Some(tokens::Token::Skipped);
                }
                if count == 1 {
                    return Some(tokens::Token::Italic(text.trim().to_string()));
                } else {
                    return Some(tokens::Token::Bold(text.trim().to_string()));
                }
            } 
            Some('!') => {
                self.advance();
                self.advance();
                let mut img_text = String::new();
                while let Some(ch) = self.peek_char() {
                    if ch == ']' {
                        break;
                    }
                    img_text.push(ch);
                    self.advance();
                }
                self.advance();
                self.advance();
                let mut image_url = String::new();
                while let Some(ch) = self.peek_char() {
                    if ch == ')' {
                        break;
                    }
                    image_url.push(ch);
                    self.advance();
                }
                self.advance();
                self.advance();
                return Some(tokens::Token::Image(img_text.trim().to_string(), image_url.trim().to_string()));
            }
            Some('[') => {
                self.advance();
                let mut link_text = String::new();
                while let Some(ch) = self.peek_char() {
                    if ch == ']' {
                        break;
                    }
                    link_text.push(ch);
                    self.advance();
                }
                self.advance();
                self.advance();
                let mut link_url = String::new();
                while let Some(ch) = self.peek_char() {
                    if ch == ')' {
                        break;
                    }
                    link_url.push(ch);
                    self.advance();
                }
                self.advance();
                return Some(tokens::Token::Link(link_text.trim().to_string(), link_url.trim().to_string()));
            }
            Some('\n') => {
                self.advance();
                return Some(tokens::Token::NewLine);
            }
            Some(' ') => {
                self.skip_whitespace();
                Some(tokens::Token::Skipped) // Return a special token for skipped tokens
            }
            Some(_) => {
                let mut paragraph_text = String::new();
                while let Some(ch) = self.peek_char() {
                    if ch == '\n' {
                        break;
                    }
                    paragraph_text.push(ch);
                    self.advance();
                }
                if paragraph_text.trim().is_empty(){
                    return Some(tokens::Token::Skipped);
                }
                return Some(tokens::Token::Paragraph(paragraph_text.trim().to_string()));
            }
        }
    }
}
