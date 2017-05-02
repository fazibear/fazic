use std::str::FromStr;
use fazic::runtime::ast::*;

pub fn entry_node(line: Option<NodeElement>, ast: Vec<NodeElement>) -> Entry {
    match line {
        None => Entry(None, ast),
        Some(NodeElement::Value(Value::Integer(line))) => Entry(Some(line), ast),
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
    NodeElement::Node(Node(Opcode::Var, vec![NodeElement::Value(Value::String(string.to_string().to_uppercase()))]))
}

pub fn variable_name(string: &str) -> NodeElement {
    NodeElement::Value(Value::String(string.to_string().to_uppercase()))
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
