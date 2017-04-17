extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod node_builder;
pub mod operators;
pub mod commands;
pub mod program;
pub mod functions;


use self::ast::{Entry, NodeElement, Node, Opcode};

pub fn exec(fazic: &mut ::fazic::Fazic) {
    let input = fazic.text_buffer.get_current_line_string();
    if input.len() == 0 {
        fazic.text_buffer.insert_line("");
        return;
    }
    fazic.text_buffer.enter();
    let ast = parser::parse_all(&input);

    match ast {
        Ok(Entry(None, nodes)) => {
            run_each_node(nodes, fazic);
            fazic.text_buffer.enter();
            fazic.text_buffer.prompt();
        },
        Ok(Entry(line, ast)) => {
            fazic.program.add_line(line.unwrap() as u16, ast, input.clone());
        }
        Err(lalrpop_util::ParseError::InvalidToken{location}) => {
            fazic.text_buffer.insert_line(&format!("{: >1$}", "^", location + 1));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
        _ => {
            fazic.text_buffer.enter();
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    }
}


fn run_each_node(nodes: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) {
    for node in nodes {
        run_node(node, fazic)
    }
}

fn run_node(node: NodeElement, fazic: &mut ::fazic::Fazic) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => commands::print(fazic, evaluate_each_node(params)),
        NodeElement::Node(Node(Opcode::List, _)) => commands::list(fazic),
        NodeElement::Node(Node(Opcode::Rem, _)) => (),
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
        NodeElement::Node(Node(Opcode::Abs, params)) => functions::abs(evaluate_each_node(params)),
        NodeElement::Value(_) => node,
        _ => NodeElement::Error("Not implemented".to_string()),
    };
}
