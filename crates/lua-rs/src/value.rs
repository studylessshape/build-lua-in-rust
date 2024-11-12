use crate::vm::ExeState;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    String(String),
    Function(fn (&mut ExeState) -> i32),
}

// impl fmt::Debug for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Nil => write!(f, "Nil"),
//             Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
//             Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
//         }
//     }
// }