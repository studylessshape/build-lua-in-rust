use std::collections::HashMap;

use crate::{byte_code::ByteCode, parse::ParseProto, value::Value};

#[derive(Debug)]
pub struct ExeState {
    global: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl ExeState {
    pub fn new() -> Self {
        let mut global = HashMap::new();
        global.insert(String::from("print"), Value::Function(lib_print));

        Self {
            global,
            stack: Vec::new(),
        }
    }

    pub fn set_stack(&mut self, dst: u8, v: Value) {
        self.stack.insert(dst as usize, v);
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
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}");
                    }
                }
            }
        }
    }
}

/// "print" function in Lua's std-lib.
///
/// It supports only 1 argument and assumes the argument is at index:1 on stack.
pub fn lib_print(state: &mut ExeState) -> i32 {
    println!("{:?}", state.stack[1]);
    0
}
