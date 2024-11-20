#[derive(Debug)]
pub enum ByteCode{
    SetGlobal(u8, u8),
    GetGlobal(u8, u8),
    SetGlobalGlobal(u8, u8),
    SetGlobalConst(u8, u8),
    LocalConst(u8, u8),
    LoadNil(u8),
    LoadBool(u8, bool),
    LoadInt(u8, i16),
    Move(u8, u8),
    Call(u8, u8),
}
