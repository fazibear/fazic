use ::fazic::nodes::Node;

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
    Next(usize, usize, usize, usize)
}

#[derive(Debug)]
pub enum Instruction {
    Stop,
    Push(Stack),
    Pop,
    Jmp(usize),
    JmpIf(usize, usize),
    Set(usize, Value),

    Run,
    List,
    Clr,
    New,
    Load(usize),
    Save(usize),
    Next,
    Flip,
    Print(usize),
    Dot(usize, usize),
    Mode(u8),
    Color(usize),

    Mov(usize, usize),

    Add(usize, usize, usize),
    Gt(usize, usize, usize),
    Lt(usize, usize, usize),
    LtEq(usize, usize, usize),

}
