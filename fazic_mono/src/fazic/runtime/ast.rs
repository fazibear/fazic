#[derive(Debug)]
pub struct Entry(pub Option<i32>, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum NodeElement {
    Node(Node),
    Value(Value),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Node(pub Opcode, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Var(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Var,
    Let,

    //
    Mul,
    Div,
    Add,
    Sub,
    Or,
    And,

    // Functions
    Abs,
    Neg,

    // Commands
    List,
    Print,
    Rem,
    Run,
    End,
    Goto,

    Gosub,
    Return,
}
