use std::fmt;

use crate::vm::ExeState;

#[derive(Clone)]
pub enum Value {
    Nil,
    String(String),
    Function(fn (&mut ExeState) -> i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::String(s) => write!(f, "{s}"),
            Self::Function(_func) => write!(f, "function"),
        }
    }
}