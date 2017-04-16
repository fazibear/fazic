pub mod operators;
pub mod commands;

use fazic::ast::{Entry, NodeElement, Node, Opcode};

pub fn exec(entry: Entry) {
    let Entry(line, nodes) = entry;

    if line.is_some() {
        println!("with line");
    } else {
        run_each_node(nodes);
    }
}

fn run_each_node(nodes: Vec<NodeElement>) {
    for node in nodes {
        run_node(node)
    }
}

fn run_node(node: NodeElement) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => commands::print(evaluate_each_node(params)),
        NodeElement::Node(_) => println!("ups! node!"),
        NodeElement::Value(_) => println!("ups! value!"),
        NodeElement::Error(e) => println!("ERROR: {}", e),
    }
}

fn evaluate_each_node(nodes: Vec<NodeElement>) -> Vec<NodeElement> {
    nodes
        .into_iter()
        .map(evaluate_node)
        .collect()
}

fn evaluate_node(node: NodeElement) -> NodeElement {
    return match node {
        NodeElement::Node(Node(Opcode::Add, params)) => operators::add(evaluate_each_node(params)),
        NodeElement::Node(Node(Opcode::Sub, params)) => operators::sub(evaluate_each_node(params)),
        NodeElement::Node(Node(Opcode::Mul, params)) => operators::mul(evaluate_each_node(params)),
        NodeElement::Node(Node(Opcode::Div, params)) => operators::div(evaluate_each_node(params)),
        NodeElement::Node(_) => NodeElement::Error("Not implemented".to_string()),
        NodeElement::Value(_) => node,
        NodeElement::Error(_) => node,
    };
}
