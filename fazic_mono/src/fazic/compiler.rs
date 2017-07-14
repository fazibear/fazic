use ::fazic::enums::*;
use ::fazic::nodes::Node;

fn match_node(name: &str, params: Vec<NodeElement>) -> Vec<Instruction> {
    match name {
        "run" => vec![Instruction::Run],
        "print" => {
            let mut instructions: Vec<Instruction> = vec![];
            let val = match params[0] {
                NodeElement::Value(ref v) => v,
                _ => unreachable!()
            };

            instructions.push(Instruction::SetVar(0, val.clone()));
            instructions.push(Instruction::Print(0));
            instructions.push(Instruction::Stop);
            instructions
        }
        _ => {
            println!("Can't translate: {}", name);
            vec![]
        }
    }
}

fn match_value(value: Value) -> Vec<Instruction> {
    vec![]
}

fn match_error(error: String) -> Vec<Instruction> {
    vec![]
}

fn translate(node: NodeElement) -> Vec<Instruction> {
    match node {
        NodeElement::Node(Node(str, params)) => match_node(str.as_ref(), params),
        NodeElement::Value(v) => match_value(v),
        NodeElement::Error(s) => match_error(s),
    }
}

pub fn compile(nodes: Vec<NodeElement>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for node in nodes {
        instructions.extend(translate(node))
    }

    instructions
}
