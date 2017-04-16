pub mod operators;
pub mod commands;

use fazic::ast::{NodeElement, Node, Opcode};

pub fn run(node: NodeElement) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => commands::print(evaluate(params)),
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
    return match node {
        NodeElement::Node(Node(Opcode::Add, params)) => operators::add(evaluate(params)),
        NodeElement::Node(Node(Opcode::Sub, params)) => operators::sub(evaluate(params)),
        NodeElement::Node(Node(Opcode::Mul, params)) => operators::mul(evaluate(params)),
        NodeElement::Node(Node(Opcode::Div, params)) => operators::div(evaluate(params)),
        NodeElement::Node(_) => NodeElement::Error("Not implemented".to_string()),
        NodeElement::Value(_) => node,
        NodeElement::Error(_) => node,
    };
}
