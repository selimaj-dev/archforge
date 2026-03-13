pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Instruction>,
}

pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}

pub enum Operand {
    Global(String),
    Local(String),
    Number(i64),
    Pointer(Box<Operand>),
}