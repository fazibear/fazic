use fazic::runtime::ast::{NodeElement, Value};

pub fn abs(mut params: Vec<NodeElement>) -> NodeElement {
    let param = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match param {
        Some(NodeElement::Value(Value::Integer(val))) => NodeElement::Value(Value::Integer(val.abs())),
        Some(NodeElement::Value(Value::Float(val))) => NodeElement::Value(Value::Float(val.abs())),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),

    }
}

pub fn neg(mut params: Vec<NodeElement>) -> NodeElement {
    let param = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match param {
        Some(NodeElement::Value(Value::Integer(val))) => NodeElement::Value(Value::Integer(val * -1)),
        Some(NodeElement::Value(Value::Float(val))) => NodeElement::Value(Value::Float(val * -1.0)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),

    }
}
