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
    Bool(bool),
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
    Pow,
    Or,
    And,
    Not,

    //
    Eql,
    NotEql,
    Lt,
    Gt,
    GtEql,
    LtEql,

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

    For,
    Next,

    Gosub,
    Return,

    If,
}
