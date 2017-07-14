use ::fazic::enums::*;
use ::fazic::nodes::Node;

fn match_node(instructions: &mut Vec<Instruction>, name: &str, params: &[NodeElement]) {
    match name {
        "run" => instructions.push(Instruction::Run),
        "print" => {
            let val = match params[0] {
                NodeElement::Value(ref v) => v,
                _ => unreachable!()
            };

            instructions.push(Instruction::SetVar(0, val.clone()));
            instructions.push(Instruction::Print(0));
        }
        _ => {
            println!("Can't translate: {}", name);
        }
    };
}

fn match_value(value: &Value) {
    ()
}

fn match_error(error: &str) {
    ()
}

fn translate(instructions: &mut Vec<Instruction>, node: &NodeElement) {
    match *node {
        NodeElement::Node(Node(ref str, ref params)) => match_node(instructions, str, params),
        NodeElement::Value(ref v) => match_value(v),
        NodeElement::Error(ref s) => match_error(s),
    };
}

pub fn compile(nodes: &[NodeElement]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for node in nodes {
        translate(&mut instructions, &node)
    }

    instructions.push(Instruction::Stop);
    instructions
}
