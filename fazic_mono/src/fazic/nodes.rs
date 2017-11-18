use std::str::FromStr;
use fazic::enums::*;

#[derive(Debug)]
pub struct Entry(pub Option<i32>, pub Vec<NodeElement>);

#[derive(Debug, Clone)]
pub struct Node(pub String, pub Vec<NodeElement>);

pub fn entry_node(line: &Option<NodeElement>, ast: Vec<Vec<NodeElement>>) -> Entry {
    let mut flat_ast = vec![];
    for node in ast {
        flat_ast.extend(node);
    }

    match *line {
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
    NodeElement::Var(string.to_string().to_uppercase())
}

pub fn node(name: &str, vec: Vec<NodeElement>) -> NodeElement {
    NodeElement::Node(Node(name.to_string(), vec))
}

pub fn nodes(name: &str, vec: Vec<NodeElement>) -> Vec<NodeElement> {
    vec![node(name, vec)]
}
