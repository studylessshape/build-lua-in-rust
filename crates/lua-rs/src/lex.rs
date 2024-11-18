use std::{char, fs::File, io::{Read, Seek}};

use crate::token::Token;

#[macro_export]
macro_rules! match_string {
    ($str:ident, $other1:expr => $run1:stmt $(, $other:expr => $run:stmt)*) => {
        if $str == $other1 {
            $run1
        } $(else if $str == $other {
            $run
        })*
    };
}

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
        if let Ok(ch) = self.read_char() {
            match ch {
                ' ' | '\r' | '\n' | '\t' => return self.next(),
                '\0' => return Token::EOF,
                '\"' => return self.read_string(),
                '+' => return Token::Add,
                '-' => return Token::Sub,
                '*' => return Token::Mul,
                '/' => return self.read_div(),
                '^' => return Token::Pow,
                '#' => return Token::Len,
                '&' => return Token::BitAnd,
                '~' => return self.read_bitxor(),
                '|' => return Token::BitOr,
                '<' => return self.read_less(),
                '>' => return self.read_greater(),
                '=' => return self.read_assign(),
                '(' => return Token::ParL,
                ')' => return Token::ParR,
                '{' => return Token::CurlyL,
                '}' => return Token::CurlyR,
                '[' => return Token::SqurL,
                ']' => return Token::SqurR,
                ';' => return Token::SemiColon,
                ':' => return self.read_colon(),
                ',' => return Token::Comma,
                '.' => return self.read_dot(),
                _ => return self.read_name(ch)
            }
        }

        Token::EOF
    }

    fn read_string(&mut self) -> Token {
        let mut str = String::new();
        while let Ok(ch) = self.read_char() {
            match ch {
                '\"' => if str.ends_with('\\') {
                    str.push(ch);
                } else {
                    break;
                },
                _ => str.push(ch),
            }
        }

        Token::String(str)
    }

    fn read_name(&mut self, ch: char) -> Token {
        let mut str = ch.to_string();
        while let Ok(ch) = self.read_char() {
            if ch.is_ascii_alphanumeric() || ch == '.' {
                str.push(ch)
            } else {
                self.back_seek();
                break;
            }
        }

        match_string!(str,
            "and" => return Token::And,
            "break" => return Token::Break,
            "do" => return Token::Do,
            "else" => return Token::Else,
            "elseif" => return Token::ElseIf,
            "end" => return Token::End,
            "false" => return Token::False,
            "for" => return Token::For,
            "function" => return Token::Function,
            "goto" => return Token::Goto,
            "if" => return Token::If,
            "in" => return Token::In,
            "local" => return Token::Local,
            "nil" => return Token::Nil,
            "not" => return Token::Not,
            "or" => return Token::Or,
            "repeat" => return Token::Repeat,
            "return" => return Token::Return,
            "Then" => return Token::Then,
            "True" => return Token::True,
            "Until" => return Token::Until,
            "While" => return Token::While
        );

        if let Some(ch) = str.chars().next() {
            if ch.is_ascii_digit() {
                if str.contains('.') {
                    return Token::Float(str.parse::<f64>().expect("Invalid Float!"));
                } else {
                    return Token::Integer(str.parse::<i64>().expect("Invalid Integer!"));
                }
            }
        }

        Token::Name(str)
    }

    fn read_bitxor(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            if ch == '=' {
                return Token::NotEq;
            } else {
                self.back_seek();
            }
        }

        Token::BitXor
    }

    fn read_assign(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            if ch == '=' {
                return Token::Equal;
            } else {
                self.back_seek();
            }
        }
        
        Token::Assign
    }

    fn read_div(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            if ch == '/' {
                return Token::Idiv;
            } else {
                self.back_seek();
            }
        }

        Token::Div
    }

    fn read_less(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            match ch {
                '=' => return Token::LesEq,
                '<' => return Token::ShiftL,
                _ => self.back_seek(),
            }
        }

        Token::Less
    }

    fn read_greater(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            match ch {
                '=' => return Token::GreEq,
                '>' => return Token::ShiftR,
                _ => self.back_seek(),
            }
        }

        Token::Greater
    }

    fn read_dot(&mut self) -> Token {
        let mut dot_cnt = 1;
        while let Ok(ch) = self.read_char() {
            if ch == '.' {
                dot_cnt += 1;
            } else {
                self.back_seek();
                break;
            }
        }
        match dot_cnt {
            1 => Token::Dot,
            2 => Token::Concat,
            3 => Token::Dots,
            _ => panic!("Unexpected dot!"),
        }
    }

    fn read_colon(&mut self) -> Token {
        if let Ok(ch) = self.read_char() {
            if ch == ':' {
                return Token::DoubleColon;
            } else {
                self.back_seek();
            }
        }
        Token::Colon
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

    fn back_seek(&mut self) {
        let _ = self.input.seek(std::io::SeekFrom::Current(-1));
    }
}