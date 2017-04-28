use fazic::runtime::ast::{NodeElement, Value};

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

pub fn neg(params: Vec<NodeElement>) -> NodeElement {
    if params.len() != 1 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match params[0] {
        NodeElement::Value(Value::Integer(val)) => NodeElement::Value(Value::Integer(val * -1)),
        NodeElement::Value(Value::Float(val)) => NodeElement::Value(Value::Float(val * -1.0)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
