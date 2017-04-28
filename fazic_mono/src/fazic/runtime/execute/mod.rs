pub mod operators;
pub mod commands;
pub mod functions;

use ::fazic::runtime::ast::*;

pub fn exec_each_node(nodes: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) {
    for node in nodes {
        exec_node(node, fazic)
    }
}

pub fn exec_node(node: NodeElement, fazic: &mut ::fazic::Fazic) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => commands::print(fazic, eval_each_node(params)),
        NodeElement::Node(Node(Opcode::List, _)) => commands::list(fazic),
        NodeElement::Node(Node(Opcode::Run, _)) => commands::run(fazic),
        NodeElement::Node(Node(Opcode::Rem, _)) => (),
        NodeElement::Node(Node(Opcode::Goto, params)) => commands::goto(fazic, params),
        NodeElement::Node(_) => (),
        NodeElement::Value(_) => (),
        NodeElement::Error(e) => println!("ERROR: {}", e),
    }
}

pub fn eval_each_node(nodes: Vec<NodeElement>) -> Vec<NodeElement> {
    nodes
        .into_iter()
        .map(eval_node)
        .collect()
}

pub fn eval_node(node: NodeElement) -> NodeElement {
    return match node {
        NodeElement::Node(Node(Opcode::Var, params)) => var(params),
        NodeElement::Node(Node(Opcode::Add, params)) => operators::add(eval_each_node(params)),
        NodeElement::Node(Node(Opcode::Sub, params)) => operators::sub(eval_each_node(params)),
        NodeElement::Node(Node(Opcode::Mul, params)) => operators::mul(eval_each_node(params)),
        NodeElement::Node(Node(Opcode::Div, params)) => operators::div(eval_each_node(params)),
        NodeElement::Node(Node(Opcode::Abs, params)) => functions::abs(eval_each_node(params)),
        NodeElement::Node(Node(Opcode::Neg, params)) => functions::neg(eval_each_node(params)),
        NodeElement::Value(_) => node,
        _ => NodeElement::Error("Not implemented".to_string()),
    };
}

fn var(node: Vec<NodeElement>) -> NodeElement {
    NodeElement::Value(Value::String("FROM_VAR".to_string()))
}
