use crate::apis::API;

pub enum Instruction {
    X86,
}

pub enum Architecture {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

pub struct Code {
    pub api: API,
    pub arch: Architecture,
    pub bytes: Vec<u8>,
    pub set: Instruction,
}