use ::fazic::enums::*;
use ::fazic::nodes::Node;

fn process_node(instructions: &mut Vec<Instruction>, name: &str, params: &[NodeElement]) {
    process_nodes(instructions, params);

    match name {
        "run" => instructions.push(Instruction::Run),
        "print" => instructions.push(Instruction::Print(0)),
        "add" => instructions.push(Instruction::Add(0,1,0)),
        _ => {
            println!("Can't translate: {}", name);
        }
    };
}

fn process_nodes(instructions: &mut Vec<Instruction>, nodes: &[NodeElement]) {
    for (i, node) in nodes.iter().enumerate() {
        match *node {
            NodeElement::Node(Node(ref str, ref params)) => {
                process_node(instructions, str, params);
            }
            NodeElement::Value(ref val) => {
                instructions.push(Instruction::SetVar(i, val.clone()));
            }
        }
    };
}

pub fn compile(nodes: &[NodeElement]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    process_nodes(&mut instructions, nodes);

    instructions.push(Instruction::Stop);
    instructions
}
