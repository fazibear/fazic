use std::str::FromStr;
use fazic::runtime::ast::{Entry, NodeElement, Node, Value, Opcode};

pub fn string_node(string: &str) -> NodeElement {
    let mut string: String = string.to_string();
    let len = string.len() - 1;
    let naked = string.drain(1..len).collect();
    NodeElement::Value(Value::String(naked))
}

pub fn entry_node(line: Option<NodeElement>, ast: Vec<NodeElement>) -> Entry {
    match line {
        None => Entry(None, ast),
        Some(NodeElement::Value(Value::Integer(line))) => Entry(Some(line), ast),
        _ => unreachable!(),
    }
}

pub fn integer_node(string: &str) -> NodeElement {
    NodeElement::Value(Value::Integer(i32::from_str(string).unwrap()))
}

pub fn float_node(string: &str) -> NodeElement {
    NodeElement::Value(Value::Float(f64::from_str(string).unwrap()))
}

pub fn param_0_node(opcode: Opcode) -> NodeElement {
    NodeElement::Node(Node(opcode, vec![]))
}

pub fn param_1_node(opcode: Opcode, one: NodeElement) -> NodeElement {
    NodeElement::Node(Node(opcode, vec![one]))
}

pub fn param_2_node(opcode: Opcode, one: NodeElement, two: NodeElement) -> NodeElement {
    NodeElement::Node(Node(opcode, vec![one, two]))
}
