#[derive(Debug)]
pub struct Node(pub Opcode, pub Vec<Value>);

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Node(Box<Node>),
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    // Value
    Val,

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
