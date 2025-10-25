#[derive(Debug)]
pub enum Instruction {
    Inc(usize),
    Dec(usize),
    IncMemPtr(usize),
    DecMemPtr(usize),
    GetChar(usize),
    PutChar(usize),
    JmpToIfZero(usize),
    JmpBackIfNonZero(usize),
}
