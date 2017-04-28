use fazic::runtime::ast::{NodeElement, Value};

pub fn add(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l + r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l + r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l + r as f64)),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l as f64 + r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => {
            let mut str = l.clone();
            str.push_str(&r);
            NodeElement::Value(Value::String(str))
        },
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

pub fn sub(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l - r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l - r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l - r as f64)),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l as f64 - r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

pub fn mul(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l * r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l * r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l * r as f64)),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l as f64 * r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

pub fn div(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (   _,
            Some(NodeElement::Value(Value::Integer(0)))
        ) => NodeElement::Error("?DIVISION BY ZERO".to_string()),
        (   _,
            Some(NodeElement::Value(Value::Float(0.0_f64)))
        ) => NodeElement::Error("?DIVISION BY ZERO".to_string()),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l as f64 / r as f64)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l as f64 / r as f64)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l as f64 / r as f64)),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l as f64 / r as f64)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
