#[derive(Debug)]
pub enum ByteCode{
    GetGlobal(u8, u8),
    LocalConst(u8, u8),
    Call(u8, u8)
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    String(String),
    Function(fn (&mut i32) -> i32),
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

#[cfg(test)]
mod tests {
}
