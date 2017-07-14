use ::fazic::enums::*;
use ::fazic::nodes::Node;

fn match_string_node(name: &str, params: Vec<NodeElement>) -> Vec<Instruction> {
    match name {
        "runo" => vec![Instruction::Run],
        _ => vec![Instruction::Stop],
    }
}

fn translate(node: NodeElement) -> Vec<Instruction> {
    match node {
        NodeElement::Node(Node(str, params)) => match_string_node(str.as_ref(), params),
        _ => {
            println!("Error translate node {:?}", node);
            vec![]
        }
    }
}


pub fn compile(nodes: Vec<NodeElement>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for node in nodes {
        instructions.extend(translate(node))
    }

    instructions
}
