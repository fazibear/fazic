#[derive(Debug)]
pub struct Entry(pub Option<i32>, pub Vec<NodeElement>);

#[derive(Debug)]
pub enum NodeElement {
    Node(Node),
    Value(Value),
    Error(String),
}

#[derive(Debug)]
pub struct Node(pub Opcode, pub Vec<NodeElement>);

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    //
    Mul,
    Div,
    Add,
    Sub,
    Or,
    And,

    // Functions
    Abs,

    // Commands
    Print
}
