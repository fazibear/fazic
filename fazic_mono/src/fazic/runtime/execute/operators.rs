use fazic::runtime::ast::{NodeElement, Value};

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

pub fn pow(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l.pow(r as u32))),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float(l.powf(r))),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Float(l.powf(r as f64))),
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Float((l as f64).powf(r as f64))),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

pub fn eql(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l == r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l == r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l == r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn not_eql(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l != r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l != r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l != r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn lt(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l < r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l < r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l < r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn gt(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l > r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l > r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l > r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn lt_eql(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn gt_eql(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        (
            Some(NodeElement::Value(Value::Float(l))),
            Some(NodeElement::Value(Value::Float(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::Bool(l <= r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}


pub fn and(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l & r)),
        (
            Some(NodeElement::Value(Value::Bool(l))),
            Some(NodeElement::Value(Value::Bool(r)))
        ) => NodeElement::Value(Value::Bool(l && r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn or(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l | r)),
        (
            Some(NodeElement::Value(Value::Bool(l))),
            Some(NodeElement::Value(Value::Bool(r)))
        ) => NodeElement::Value(Value::Bool(l || r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
pub fn not(params: Vec<NodeElement>) -> NodeElement {
    if params.len() != 1 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match params[0] {
        NodeElement::Value(Value::Bool(val)) => NodeElement::Value(Value::Bool(!val)),
        NodeElement::Value(Value::Integer(val)) => NodeElement::Value(Value::Integer(!val)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
