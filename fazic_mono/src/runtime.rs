use ast::{NodeElement, Node, Value, Opcode};

pub fn run(node: NodeElement) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => command_print(evaluate(params)),
        NodeElement::Node(_) => println!("ups! node!"),
        NodeElement::Value(_) => println!("ups! value!"),
        NodeElement::Error(e) => println!("ERROR: {}", e),
    }
}

fn evaluate(nodes: Vec<NodeElement>) -> Vec<NodeElement> {
    nodes
        .into_iter()
        .map(eval)
        .collect()
}

fn eval(node: NodeElement) -> NodeElement {
    let x = match node {
        NodeElement::Node(Node(Opcode::Add, params)) => operator_add(evaluate(params)),
        NodeElement::Node(Node(Opcode::Sub, params)) => operator_sub(evaluate(params)),
        NodeElement::Node(Node(Opcode::Mul, params)) => operator_mul(evaluate(params)),
        NodeElement::Node(Node(Opcode::Div, params)) => operator_div(evaluate(params)),
        NodeElement::Node(_) => NodeElement::Error("Not implemented".to_string()),
        NodeElement::Value(_) => node,
        NodeElement::Error(_) => node
    };
    return x;
}

// -------------------


fn command_print(mut params: Vec<NodeElement>) {
    let param = params.pop();

    match param {
        Some(NodeElement::Value(Value::String(s))) => println!("print: {}", s),
        Some(NodeElement::Value(Value::Integer(i))) => println!("print: {}", i),
        Some(NodeElement::Value(Value::Float(f))) => println!("print: {}", f),

        Some(_) => println!("error"),
        None => println!(""),
    }
}

fn operator_add(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l+r)),
        (
            Some(NodeElement::Value(Value::String(l))),
            Some(NodeElement::Value(Value::String(r)))
        ) => NodeElement::Value(Value::String(format!("{}{}", l, r))),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

fn operator_sub(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l-r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

fn operator_mul(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l*r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}

fn operator_div(mut params: Vec<NodeElement>) -> NodeElement {
    let right = params.pop();
    let left = params.pop();

    if params.len() != 0 {
        return NodeElement::Error("?SYNTAX ERROR".to_string());
    }

    match (left, right) {
        (
            Some(NodeElement::Value(Value::Integer(l))),
            Some(NodeElement::Value(Value::Integer(r)))
        ) => NodeElement::Value(Value::Integer(l/r)),
        _ => NodeElement::Error("?TYPE MISMATCH".to_string()),
    }
}
