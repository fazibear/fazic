use fazic::nodes::Node;

#[derive(Debug)]
pub enum Param {
    Node(usize),
    Variable(usize),
    Value(Value),
}

#[derive(Debug, Clone)]
pub enum NodeElement {
    Node(Node),
    Value(Value),
    Var(String),
    LineNo(u16),
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Stack {
    Next(usize, usize, usize, usize),
    Return(usize),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    JmpLine(u16),
    JmpIfNotNextLine(usize, u16),

    End,
    Push(Stack),
    Pop,
    Jmp(usize),
    JmpIfNot(usize, usize),
    Set(usize, Value),

    Error(String),

    Run,
    List,
    Clr,
    New,
    Load(usize),
    Save(usize),
    Next,
    Return,
    Flip,
    Print(usize),
    Dot(usize, usize),
    Mode(u8),
    Color(usize),

    Mov(usize, usize),

    And(usize, usize, usize),
    Or(usize, usize, usize),
    Add(usize, usize, usize),

    Eq(usize, usize, usize),

    Gt(usize, usize, usize),
    Lt(usize, usize, usize),
    LtEq(usize, usize, usize),
    Neg(usize, usize),
    Not(usize, usize),

    Rng(usize, usize),
    Abs(usize, usize),
    Sin(usize, usize),
    Cos(usize, usize),
    Tan(usize, usize),
}
