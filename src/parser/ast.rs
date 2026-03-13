#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Instruction(Instruction),
    PointerAssign { left: Reg, right: Reg },
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Operand>,
}

#[derive(Debug)]
pub enum Operand {
    Reg(Reg),
    Number(i64),
    Pointer(Box<Operand>),
}

#[derive(Debug)]
pub enum Reg {
    Global(String),
    Local(String),
}
