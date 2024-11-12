#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    EOF,
}

impl Token {
    pub fn push_char(&mut self, ch: char) {
        match self {
            Token::Name(name) => name.push(ch),
            Token::String(str) => str.push(ch),
            Token::EOF => {},
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Token::Name(name) => name.len(),
            Token::String(str) => str.len(),
            Token::EOF => 0,
        }
    }
}