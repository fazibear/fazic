use std::str::FromStr;

#[derive(Debug)]
pub struct Entry(pub Option<i32>, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum NodeElement {
    Node(Node),
    Value(Value),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Node(String, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Bool(bool),
    Var(String),
    Null,
}

pub fn entry_node(line: Option<NodeElement>, ast: Vec<Vec<NodeElement>>) -> Entry {
    let mut flat_ast = vec![];
    for node in ast.into_iter() {
        flat_ast.extend(node);
    }

    match line {
        None => Entry(None, flat_ast),
        Some(NodeElement::Value(Value::Integer(line))) => Entry(Some(line), flat_ast),
        _ => unreachable!(),
    }
}

pub fn string_node(string: &str) -> NodeElement {
    NodeElement::Value(Value::String(string.to_string()))
}

pub fn integer_node(string: &str) -> NodeElement {
    NodeElement::Value(Value::Integer(i32::from_str(string).unwrap()))
}

pub fn float_node(string: &str) -> NodeElement {
    NodeElement::Value(Value::Float(f64::from_str(string).unwrap()))
}

pub fn variable_node(string: &str) -> NodeElement {
    NodeElement::Node(Node("var".to_string(), vec![NodeElement::Value(Value::String(string.to_string().to_uppercase()))]))
}

pub fn variable_name(string: &str) -> NodeElement {
    NodeElement::Value(Value::String(string.to_string().to_uppercase()))
}

pub fn node(name: &str, vec: Vec<NodeElement>) -> NodeElement {
    NodeElement::Node(Node(name.to_string(), vec))
}

pub fn nodes(name: &str, vec: Vec<NodeElement>) -> Vec<NodeElement> {
    vec![node(name, vec)]
}
