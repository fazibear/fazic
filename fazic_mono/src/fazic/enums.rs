use ::fazic::nodes::Node;

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
    SetVar(usize, Value),

    Run,
    List,
    Next,
    Flip,
    Print(usize),
    Dot(usize, usize),
    Mode(u8),
    Color(usize),

    Add(usize, usize, usize),
    Gt(usize, usize, usize),
    Lt(usize, usize, usize),
    LtEq(usize, usize, usize),

}
