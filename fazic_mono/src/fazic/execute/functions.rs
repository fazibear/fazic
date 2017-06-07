use fazic::ast::{NodeElement, Value};
extern crate rand;

pub fn abs(params: Vec<NodeElement>) -> NodeElement {
    if params.len() != 1 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match params[0] {
        NodeElement::Value(Value::Integer(val)) => NodeElement::Value(Value::Integer(val.abs())),
        NodeElement::Value(Value::Float(val)) => NodeElement::Value(Value::Float(val.abs())),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

pub fn rnd(params: Vec<NodeElement>) -> NodeElement {
    if params.len() != 1 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }
    let rand = rand::random::<f64>();

    NodeElement::Value(Value::Float(rand))
}
