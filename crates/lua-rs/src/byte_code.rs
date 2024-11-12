#[derive(Debug)]
pub enum ByteCode{
    GetGlobal(u8, u8),
    LocalConst(u8, u8),
    Call(u8, u8)
}
