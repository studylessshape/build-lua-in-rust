use std::collections::HashMap;

use crate::{byte_code::ByteCode, parse::ParseProto, value::Value};

#[derive(Debug)]
pub struct ExeState {
    global: HashMap<String, Value>,
    stack: Vec<Value>,
    func_index: usize,
}

impl Default for ExeState {
    fn default() -> Self {
        Self::new()
    }
}

impl ExeState {
    pub fn new() -> Self {
        let mut global = HashMap::new();
        global.insert(String::from("print"), Value::Function(lib_print));

        Self {
            global,
            stack: Vec::new(),
            func_index: 0,
        }
    }

    pub fn set_stack(&mut self, dst: u8, v: Value) {
        let dst = dst as usize;
        match dst.cmp(&self.stack.len()) {
            std::cmp::Ordering::Equal => self.stack.push(v),
            std::cmp::Ordering::Less => self.stack[dst] = v,
            std::cmp::Ordering::Greater => panic!("fail in stack!"),
        }
    }

    pub fn execute(&mut self, proto: &ParseProto) {
        for code in proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];

                    if let Value::String(key) = name {
                        let v = self.global.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LocalConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    self.func_index = func as usize;
                    let func = &self.stack[self.func_index];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}");
                    }
                }
                ByteCode::LoadNil(dst) => self.set_stack(dst, Value::Nil),
                ByteCode::LoadBool(dst, v) => self.set_stack(dst, Value::Boolean(v)),
                ByteCode::LoadInt(dst, v) => self.set_stack(dst, Value::Integer(v.into())),
                ByteCode::Move(dst, v) => {
                    let v = proto.constants[v as usize].clone();
                    self.set_stack(dst, v);
                },
            }
        }
    }
}

/// "print" function in Lua's std-lib.
///
/// It supports only 1 argument and assumes the argument is at index:1 on stack.
pub fn lib_print(state: &mut ExeState) -> i32 {
    println!("{:?}", state.stack[state.func_index + 1]);
    0
}
