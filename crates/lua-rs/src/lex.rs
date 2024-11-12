use std::{char, fs::File, io::Read};

use crate::token::Token;

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Self {
            input
        }
    }

    pub fn next(&mut self) -> Token {
        while let Ok(ch) = self.read_char() {
            match ch {
                ' ' | '\r' | '\n' | '\t' => return self.next(),
                '\0' => return Token::EOF,
                '\"' => return self.token_string(),
                _ => return self.token_name(ch)
            }
        }

        Token::EOF
    }

    fn token_string(&mut self) -> Token {
        let mut str = String::new();
        while let Ok(ch) = self.read_char() {
            match ch {
                '\"' => break,
                _ => str.push(ch),
            }
        }

        Token::String(str)
    }

    fn token_name(&mut self, ch: char) -> Token {
        let mut str = ch.to_string();
        while let Ok(ch) = self.read_char() {
            match ch {
                ' ' | '\r' | '\n' | '\t' | '\0' => break,
                _ => str.push(ch),
            }
        }

        Token::Name(str)
    }

    fn read_char(&mut self) -> std::io::Result<char> {
        let mut buf: [u8; 1] = [0];
        let size = self.input.read(&mut buf)?;
        if size == 1 {
            Ok(buf[0] as char)
        } else {
            Ok('\0')
        }
    }
}