use std::{
    char,
    fs::File,
    io::{Error, Read, Result, Seek},
};

use crate::token::Token;

#[derive(Debug)]
pub struct Lex {
    input: File,
    ahead: Option<Token>
}

impl Lex {
    pub fn new(input: File) -> Self {
        Self { input, ahead: None }
    }

    pub fn next(&mut self) -> Result<Token> {
        match self.ahead.take() {
            None | Some(Token::EOF) => self.do_next(),
            Some(token) => Ok(token),
        }
    }

    pub fn peak(&mut self) -> Result<&Option<Token>> {
        if None == self.ahead || Some(Token::EOF) == self.ahead {
            self.ahead = Some(self.do_next()?);
        }

        Ok(&self.ahead)
    }

    fn do_next(&mut self) -> Result<Token> {
        while let Ok(ch) = self.read_char() {
            return match ch {
                ' ' | '\r' | '\n' | '\t' => continue,
                '\0' => Ok(Token::EOF),
                '"' | '\'' => self.read_string(ch),
                '+' => Ok(Token::Add),
                '-' => self.ahead_char(
                    '-',
                    |lex| {
                        lex.read_comment();
                        lex.do_next()
                    },
                    |_| Ok(Token::Sub),
                ),
                '*' => Ok(Token::Mul),
                '/' => self.read_ahead('/', Token::Idiv, Token::Div),
                '^' => Ok(Token::Pow),
                '#' => Ok(Token::Len),
                '&' => Ok(Token::BitAnd),
                '~' => self.read_ahead('=', Token::NotEq, Token::BitXor),
                '|' => Ok(Token::BitOr),
                '<' => self.ahead_char(
                    '<',
                    |_| Ok(Token::ShiftL),
                    |lex| lex.read_ahead('=', Token::LesEq, Token::Less),
                ),
                '>' => self.ahead_char(
                    '>',
                    |_| Ok(Token::ShiftR),
                    |lex| lex.read_ahead('=', Token::GreEq, Token::Greater),
                ),
                '=' => self.read_ahead('=', Token::Equal, Token::Assign),
                '(' => Ok(Token::ParL),
                ')' => Ok(Token::ParR),
                '{' => Ok(Token::CurlyL),
                '}' => Ok(Token::CurlyR),
                '[' => Ok(Token::SqurL),
                ']' => Ok(Token::SqurR),
                ';' => Ok(Token::SemiColon),
                ':' => self.read_ahead(':', Token::DoubleColon, Token::Colon),
                ',' => Ok(Token::Comma),
                '.' => match self.read_char()? {
                    '.' => self.read_ahead('.', Token::Dots, Token::Concat),
                    '0'..='9' => {
                        self.back_seek()?;
                        self.read_number(ch)
                    },
                    _ => Ok(Token::Dot)
                },
                '0'..='9' => self.read_number(ch),
                'a'..='z' | 'A'..='Z' | '_' => self.read_name(ch),
                _ => Err(Error::other(format!("invalid char {ch}")))
            };
        }
        
        Ok(Token::EOF)
    }

    fn read_string(&mut self, qoute: char) -> Result<Token> {
        let mut str = String::new();
        while let Ok(ch) = self.read_char() {
            match ch {
                ch if ch == qoute => {
                    if str.ends_with('\\') {
                        str.push(ch);
                    } else {
                        break;
                    }
                },
                '\n' | '\0' => return Err(Error::other("unfinished string")),
                _ => str.push(ch),
            }
        }

        Ok(Token::String(str))
    }

    fn read_name(&mut self, ch: char) -> Result<Token> {
        let mut str = ch.to_string();
        while let Ok(ch) = self.read_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                str.push(ch)
            } else {
                self.back_seek()?;
                break;
            }
        }

        Ok(match str.as_str() {
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::ElseIf,
            "end" => Token::End,
            "false" => Token::False,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "if" => Token::If,
            "in" => Token::In,
            "local" => Token::Local,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::True,
            "until" => Token::Until,
            "while" => Token::While,
            _ => Token::Name(str)
        })
    }

    /// Use [After] to handle complex situation
    fn ahead_char<F1, F2>(&mut self, ahead: char, long: F1, short: F2) -> Result<Token>
    where
        F1: Fn(&mut Lex) -> Result<Token>,
        F2: Fn(&mut Lex) -> Result<Token>,
    {
        if ahead == self.read_char()? {
            long(self)
        } else {
            self.back_seek()?;
            short(self)
        }
    }

    /// Simple read after.
    ///
    /// Complex is [Lex::after]
    fn read_ahead(&mut self, ahead: char, long: Token, short: Token) -> Result<Token> {
        if self.read_char()? == ahead {
            Ok(long)
        } else {
            self.back_seek()?;
            Ok(short)
        }
    }

    /// skip comment char
    fn read_comment(&mut self) {
        while let Ok(ch) = self.read_char() {
            if ch == '\n' || ch == '\0' {
                break;
            }
        }
    }

    fn read_number(&mut self, first: char) -> Result<Token> {
        let mut str = first.to_string();
        while let Ok(ch) = self.read_char() {
            if ch.is_ascii_digit() || ch == '.' || ch == 'x' || ch == 'e' || ch == 'b'{
                str.push(ch)
            } else {
                self.back_seek()?;
                break;
            }
        }
        if str.contains('.') {
            return Ok(Token::Float(str.parse::<f64>().map_err(|_e| Error::other("Invalid Float"))?));
        } else {
            return Ok(Token::Integer(str.parse::<i64>().map_err(|_e| Error::other("Invalid Integer!"))?));
        }
    }

    /// read char from file stream, the cursor position will step one.
    fn read_char(&mut self) -> Result<char> {
        let mut buf: [u8; 1] = [0];
        let size = self.input.read(&mut buf)?;
        if size == 1 {
            Ok(buf[0] as char)
        } else {
            Ok('\0')
        }
    }

    /// back cursor position one
    fn back_seek(&mut self) -> std::io::Result<u64> {
        self.input.seek(std::io::SeekFrom::Current(-1))
    }
}
