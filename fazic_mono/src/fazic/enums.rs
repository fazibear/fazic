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
    Float(f32),
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
    Stop,
    Cont,
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
    Dir,
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
    Sub(usize, usize, usize),
    Mul(usize, usize, usize),
    Div(usize, usize, usize),
    Pow(usize, usize, usize),

    Eq(usize, usize, usize),
    Neg(usize, usize),
    Lt(usize, usize, usize),
    Gt(usize, usize, usize),
    LtEq(usize, usize, usize),
    GtEq(usize, usize, usize),
    Not(usize, usize),

    Rng(usize, usize),
    Abs(usize, usize),
    Sin(usize, usize),
    Cos(usize, usize),
    Tan(usize, usize),
    Atn(usize, usize),
    Exp(usize, usize),
    Log(usize, usize),
    Sqr(usize, usize),
    Sgn(usize, usize),
    Len(usize, usize),
    Chr(usize, usize),
    Asc(usize, usize),
    Val(usize, usize),
    Int(usize, usize),
    Str(usize, usize),
}
