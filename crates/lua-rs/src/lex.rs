use std::{
    char,
    fs::File,
    io::{Error, Read, Result, Seek},
};

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

struct After<'a, F1, F2> {
    /// after char need compare to
    after: char,
    lex: &'a mut Lex,
    long: F1,
    short: F2,
}

impl<'a, F1, F2> After<'a, F1, F2>
where
    F1: Fn(&mut Lex) -> Result<Token>,
    F2: Fn(&mut Lex) -> Result<Token>,
{
    pub fn new(ch: char, lex: &'a mut Lex, long: F1, short: F2) -> Self {
        Self {
            after: ch,
            lex,
            long,
            short,
        }
    }

    pub fn done(&mut self) -> Result<Token> {
        let ch = self.lex.read_char()?;
        if ch == self.after {
            (self.long)(&mut self.lex)
        } else {
            self.lex.back_seek()?;
            (self.short)(&mut self.lex)
        }
    }
}

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Self { input }
    }

    pub fn next(&mut self) -> Result<Token> {
        while let Ok(ch) = self.read_char() {
            return match ch {
                ' ' | '\r' | '\n' | '\t' => continue,
                '\0' => Ok(Token::EOF),
                '\"' => self.read_string(),
                '+' => Ok(Token::Add),
                '-' => self.after(
                    '-',
                    |lex| {
                        lex.read_comment();
                        lex.next()
                    },
                    |_| Ok(Token::Sub),
                ),
                '*' => Ok(Token::Mul),
                '/' => self.read_after('/', Token::Idiv, Token::Div),
                '^' => Ok(Token::Pow),
                '#' => Ok(Token::Len),
                '&' => Ok(Token::BitAnd),
                '~' => self.read_after('=', Token::NotEq, Token::BitXor),
                '|' => Ok(Token::BitOr),
                '<' => self.after(
                    '<',
                    |_| Ok(Token::ShiftL),
                    |lex| lex.read_after('=', Token::LesEq, Token::Less),
                ),
                '>' => self.after(
                    '>',
                    |_| Ok(Token::ShiftR),
                    |lex| lex.read_after('=', Token::GreEq, Token::Greater),
                ),
                '=' => self.read_after('=', Token::Equal, Token::Assign),
                '(' => Ok(Token::ParL),
                ')' => Ok(Token::ParR),
                '{' => Ok(Token::CurlyL),
                '}' => Ok(Token::CurlyR),
                '[' => Ok(Token::SqurL),
                ']' => Ok(Token::SqurR),
                ';' => Ok(Token::SemiColon),
                ':' => self.read_after(':', Token::DoubleColon, Token::Colon),
                ',' => Ok(Token::Comma),
                '.' => self.after(
                    '.',
                    |lex| lex.read_after('.', Token::Dots, Token::Concat),
                    |_| Ok(Token::Dot),
                ),
                '0'..='9' => self.read_number(ch),
                'a'..='z' | 'A'..='Z' | '_' => self.read_name(ch),
                _ => Err(Error::other(format!("invalid char {ch}")))
            };
        }
        
        Ok(Token::EOF)
    }

    fn read_string(&mut self) -> Result<Token> {
        let mut str = String::new();
        while let Ok(ch) = self.read_char() {
            match ch {
                '\"' => {
                    if str.ends_with('\\') {
                        str.push(ch);
                    } else {
                        break;
                    }
                }
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

        match_string!(str,
            "and" => return Ok(Token::And),
            "break" => return Ok(Token::Break),
            "do" => return Ok(Token::Do),
            "else" => return Ok(Token::Else),
            "elseif" => return Ok(Token::ElseIf),
            "end" => return Ok(Token::End),
            "false" => return Ok(Token::False),
            "for" => return Ok(Token::For),
            "function" => return Ok(Token::Function),
            "goto" => return Ok(Token::Goto),
            "if" => return Ok(Token::If),
            "in" => return Ok(Token::In),
            "local" => return Ok(Token::Local),
            "nil" => return Ok(Token::Nil),
            "not" => return Ok(Token::Not),
            "or" => return Ok(Token::Or),
            "repeat" => return Ok(Token::Repeat),
            "return" => return Ok(Token::Return),
            "then" => return Ok(Token::Then),
            "true" => return Ok(Token::True),
            "until" => return Ok(Token::Until),
            "while" => return Ok(Token::While)
        );

        Ok(Token::Name(str))
    }

    /// Use [After] to handle complex situation
    fn after<F1, F2>(&mut self, after: char, long: F1, short: F2) -> Result<Token>
    where
        F1: Fn(&mut Lex) -> Result<Token>,
        F2: Fn(&mut Lex) -> Result<Token>,
    {
        return After::new(after, self, long, short).done();
    }

    /// Simple read after.
    ///
    /// Complex is [Lex::after]
    fn read_after(&mut self, after: char, long: Token, short: Token) -> Result<Token> {
        if self.read_char()? == after {
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
