use std::fs::File;

use crate::{byte_code::ByteCode, lex::Lex, token::Token, value::Value};
use std::io::Result;

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
    locals: Vec<String>,
    lex: Lex,
}

impl ParseProto {
    pub fn load(input: File) -> Result<ParseProto> {
        let mut proto = Self {
            constants: Vec::new(),
            byte_codes: Vec::new(),
            locals: Vec::new(),
            lex: Lex::new(input)
        };

        proto.chunk()?;

        println!("constants: {:?}", &proto.constants);
        println!("byte_codes:");
        for c in proto.byte_codes.iter() {
            println!("  {:?}", c);
        }

        Ok(proto)
    }

    fn chunk(&mut self) -> Result<()> {
        loop {
            match self.lex.next()? {
                Token::Name(name) => self.function_call(name)?,
                Token::Local => self.local()?,
                Token::EOF => break,
                t => panic!("unexpected token: {t:?}")
            }
        }

        Ok(())
    }

    fn function_call(&mut self, name: String) -> Result<()> {
        let ifunc = self.locals.len();
        let iargs = ifunc + 1;

        let code = self.load_var(ifunc, name);
        self.byte_codes.push(code);

        match self.lex.next()? {
            Token::ParL => {
                self.load_exp(iargs)?;

                if self.lex.next()? != Token::ParR {
                    panic!("expectede `)`");
                }
            },
            Token::String(s) => {
                let code = self.load_const(iargs, Value::String(s));
                self.byte_codes.push(code);
            },
            _ => panic!("unexpected string!"),
        }

        self.byte_codes.push(ByteCode::Call(ifunc as u8, 1));
        Ok(())
    }

    fn local(&mut self) -> Result<()> {
        let var = if let Token::Name(var) = self.lex.next()? {
            var
        } else {
            panic!("expected variable!");
        };

        if self.lex.next()? != Token::Assign {
            panic!("expected `=`");
        }

        self.load_exp(self.locals.len())?;

        self.locals.push(var);
        Ok(())
    }

    fn add_const(&mut self, v: Value) -> usize {
        self.constants.iter()
                .position(|c| c == &v)
                .unwrap_or_else(|| {
                    self.constants.push(v);
                    self.constants.len() - 1
                })
    }

    fn load_const(&mut self, dst: usize, c: Value) -> ByteCode {
        ByteCode::LocalConst(dst as u8, self.add_const(c) as u8)
    }

    fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
        if let Some(idx) = self.locals.iter().rposition(|v| v == &name) {
            ByteCode::Move(dst as u8, idx as u8)
        } else {
            let ic = self.add_const(Value::String(name));
            ByteCode::GetGlobal(dst as u8, ic as u8)
        }
    }

    fn load_exp(&mut self, dst: usize) -> Result<()> {
        let code = match self.lex.next()? {
            Token::Nil => ByteCode::LoadNil(dst as u8),
            Token::True => ByteCode::LoadBool(dst as u8, true),
            Token::False => ByteCode::LoadBool(dst as u8, false),
            Token::Integer(i) => {
                if let Ok(ii) = i16::try_from(i) {
                    ByteCode::LoadInt(dst as u8, ii)
                } else {
                    self.load_const(dst, Value::Integer(i))
                }
            },
            Token::Float(f) => self.load_const(dst, Value::Float(f)),
            Token::String(s) => self.load_const(dst, Value::String(s)),
            Token::Name(name) => self.load_var(dst, name),
            _ => panic!("invalid argument!")
        };
    
        self.byte_codes.push(code);
        Ok(())
    }
}