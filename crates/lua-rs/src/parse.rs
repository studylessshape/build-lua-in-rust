use std::fs::File;

use crate::{byte_code::ByteCode, lex::Lex, token::Token, value::Value};
use std::io::Result;

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
    pub byte_codes: Vec<ByteCode>,
}

pub fn load(input: File) -> Result<ParseProto> {
    let mut constants = Vec::new();
    let mut byte_codes = Vec::new();
    let mut lex = Lex::new(input);
    let mut locals = Vec::new();

    loop {
        match lex.next()? {
            Token::Name(name) => {
                let ifunc = locals.len();
                let iargs = ifunc + 1;

                byte_codes.push(load_var(&mut constants, &locals, ifunc, name));

                match lex.next()? {
                    Token::ParL => {
                        load_exp(&mut byte_codes, &mut constants, &locals, lex.next()?, iargs);

                        if lex.next()? != Token::ParR {
                            panic!("expectede `)`");
                        }
                    },
                    Token::String(s) => {
                        let code = load_const(&mut constants, iargs, Value::String(s));
                        byte_codes.push(code);
                    },
                    _ => panic!("unexpected string!"),
                }

                byte_codes.push(ByteCode::Call(ifunc as u8, (ifunc + 1) as u8));
            },
            Token::Local => {
                let var = if let Token::Name(var) = lex.next()? {
                    var
                } else {
                    panic!("expected variable!");
                };

                if lex.next()? != Token::Assign {
                    panic!("expected `=`");
                }

                load_exp(&mut byte_codes, &mut constants, &locals, lex.next()?, locals.len());

                locals.push(var);
            },
            Token::EOF => break,
            t => panic!("unexpected token: {t:?}"),
        }
    }

    dbg!(&constants);
    dbg!(&byte_codes);
    Ok(ParseProto {
        constants,
        locals,
        byte_codes,
    })
}

fn add_const(constants: &mut Vec<Value>, v: Value) -> usize {
    constants.iter()
            .position(|c| c == &v)
            .unwrap_or_else(|| {
                constants.push(v);
                constants.len() - 1
            })
}

fn load_const(constants: &mut Vec<Value>, dst: usize, c: Value) -> ByteCode {
    ByteCode::LocalConst(dst as u8, add_const(constants, c) as u8)
}

fn load_exp(byte_codes: &mut Vec<ByteCode>, constants: &mut Vec<Value>, locals: &Vec<String>, token: Token, dst: usize) {
    let code = match token {
        Token::Nil => ByteCode::LoadNil(dst as u8),
        Token::True => ByteCode::LoadBool(dst as u8, true),
        Token::False => ByteCode::LoadBool(dst as u8, false),
        Token::Integer(i) => {
            if let Ok(ii) = i16::try_from(i) {
                ByteCode::LoadInt(dst as u8, ii)
            } else {
                load_const(constants, dst, Value::Integer(i))
            }
        },
        Token::Float(f) => load_const(constants, dst, Value::Float(f)),
        Token::String(s) => load_const(constants, dst, Value::String(s)),
        Token::Name(name) => load_var(constants, locals, dst, name),
        _ => panic!("invalid argument!")
    };

    byte_codes.push(code);
}

fn load_var(constants: &mut Vec<Value>, locals: &Vec<String>, dst: usize, name: String) -> ByteCode {
    if let Some(idx) = locals.iter().rposition(|v| *v == name) {
        ByteCode::Move(dst as u8, idx as u8)
    } else {
        let ic = add_const(constants, Value::String(name));
        ByteCode::GetGlobal(dst as u8, ic as u8)
    }
}