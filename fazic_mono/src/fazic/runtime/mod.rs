pub mod ast;
pub mod parser;
pub mod node_builder;
pub mod operators;
pub mod commands;
pub mod program;

use self::ast::{Entry, NodeElement, Node, Opcode};

pub fn exec(fazic: &mut ::fazic::Fazic) {
    let line = fazic.text_buffer.get_current_line_string();
    let ast = parser::parse_all(&line);
    match ast {
        Ok(Entry(None, nodes)) => run_each_node(nodes),
        Ok(Entry(line, ast)) => println!("wijt line"),
        _ => println!("Parse error"),
    }
    fazic.text_buffer.enter();
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
        _ => NodeElement::Error("Not implemented".to_string()),
    };
}
