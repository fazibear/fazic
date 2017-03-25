use std::str::FromStr;
use ast::{Node, Value, Opcode};

pub fn string_node(string: &str) -> Node {
    let mut string: String = string.to_string();
    let len = string.len() - 1;
    let naked = string.drain(1..len).collect();
    Node(Opcode::Val, vec![Value::String(naked)])
}

pub fn integer_node(string: &str) -> Node {
    Node(Opcode::Val, vec![Value::Integer(i32::from_str(string).unwrap())])
}

pub fn param_1_node(opcode: Opcode, one: Node) -> Node {
    Node(opcode, vec![Value::Node(Box::new(one))])
}

pub fn param_2_node(opcode: Opcode, one: Node, two: Node) -> Node {
    Node(opcode, vec![Value::Node(Box::new(one)), Value::Node(Box::new(two))])
}
