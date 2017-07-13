
#[derive(Debug)]
pub struct Entry(pub Option<i32>, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum NodeElement {
    Node(Node),
    Value(Value),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Node(pub String, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Bool(bool),
//    Var(String),
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
