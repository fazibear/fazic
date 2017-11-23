use std::collections::HashMap;

use fazic::enums::*;
use fazic::nodes::Node;
use fazic::variables::Variables;

fn process_param(idx: usize, params: &[Param], instructions: &mut Vec<Instruction>) -> usize {
    match params[idx] {
        Param::Node(i) => i,
        Param::Variable(var) => var,
        Param::Value(ref val) => {
            instructions.push(Instruction::Set(idx, val.clone()));
            idx
        }
    }
}

fn process_node(
    instructions: &mut Vec<Instruction>,
    name: &str,
    nodes: &[NodeElement],
    variables: &mut Variables,
    lines: &mut HashMap<u16, usize>,
    dst: usize,
) {
    let params = process_nodes(instructions, nodes, variables, lines);
    println!("{}: {:?}", name, params);

    match name {
        "run" => instructions.push(Instruction::Run),
        "list" => instructions.push(Instruction::List),
        "new" => instructions.push(Instruction::New),
        "clr" => instructions.push(Instruction::Clr),
        "flip" => instructions.push(Instruction::Flip),
        "goto" => if let Param::Value(Value::Integer(i)) = params[0] {
            instructions.push(Instruction::Goto(i as u16));
        },
        "load" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Load(p0));
        }
        "save" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Save(p0));
        }
        "color" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Color(p0));
        }
        "print" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Print(p0));
        }
        "dot" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Dot(p0, p1));
        }
        "add" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Add(p0, p1, dst));
        }
        "lt" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Lt(p0, p1, dst));
        }
        "gt" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Gt(p0, p1, dst));
        }
        "lteq" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::LtEq(p0, p1, dst));
        }
        "let" => {
            let p0 = process_param(0, &params, instructions);
            match params[1] {
                Param::Value(ref val) => instructions.push(Instruction::Set(p0, val.clone())),
                Param::Node(i) | Param::Variable(i) => instructions.push(Instruction::Mov(p0, i)),
            }
        }
        "next" => {
            instructions.push(Instruction::Next);
            instructions.push(Instruction::Pop);
        }
        "mode" => {
            if let Param::Value(Value::Integer(i)) = params[0] {
                instructions.push(Instruction::Mode(i as u8))
            };
        }
        "for" => {
            let p = process_param(0, &params, instructions);
            let max = variables.alloc(&format!("{}-MAX", p));
            let step = variables.alloc(&format!("{}-STEP", p));

            match params[1] {
                Param::Value(ref val) => instructions.push(Instruction::Set(p, val.clone())),
                Param::Node(i) | Param::Variable(i) => instructions.push(Instruction::Mov(p, i)),
            }
            match params[2] {
                Param::Value(ref val) => instructions.push(Instruction::Set(max, val.clone())),
                Param::Node(i) | Param::Variable(i) => instructions.push(Instruction::Mov(max, i)),
            }
            match params[3] {
                Param::Value(ref val) => instructions.push(Instruction::Set(step, val.clone())),
                Param::Node(i) | Param::Variable(i) => instructions.push(Instruction::Mov(step, i)),
            }
            let jmp = instructions.len() + 1;
            instructions.push(Instruction::Push(Stack::Next(p, max, step, jmp)));
        }

        "abs" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Abs(p0, dst));
        }

        "neg" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Neg(p0, dst));
        }

        _ => {
            println!("Can't translate: {}", name);
        }
    }
}

fn process_nodes(
    instructions: &mut Vec<Instruction>,
    nodes: &[NodeElement],
    variables: &mut Variables,
    lines: &mut HashMap<u16, usize>,
) -> Vec<Param> {
    let mut params: Vec<Param> = vec![];
    for (i, node) in nodes.iter().enumerate() {
        let tmp = variables.alloc(&format!("{}-TMP", i));
        match *node {
            NodeElement::Node(Node(ref str, ref nodes)) => {
                process_node(instructions, str, nodes, variables, lines, tmp);
                params.push(Param::Node(tmp));
            }
            NodeElement::Value(ref val) => {
                params.push(Param::Value(val.clone()));
            }
            NodeElement::Var(ref name) => {
                params.push(Param::Variable(variables.alloc(name)));
            }
            NodeElement::LineNo(line) => {
                lines.insert(line, instructions.len());
            }
        }
    }
    params
}

pub fn process_gotos(instruction: Instruction, lines: &HashMap<u16, usize>) -> Instruction {
    match instruction {
        Instruction::Goto(ref line) => match lines.get(line) {
            Some(pos) => Instruction::Jmp(*pos as usize),
            None => Instruction::Noop,
        },
        _ => instruction,
    }
}

pub fn compile(nodes: &[NodeElement], variables: &mut Variables) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut lines: HashMap<u16, usize> = HashMap::new();

    println!("nodes: {:?}", nodes);

    process_nodes(&mut instructions, nodes, variables, &mut lines);

    instructions.push(Instruction::Stop);

    println!("instructions: {:?}", instructions);

    instructions
        .into_iter()
        .map(|i| process_gotos(i, &lines))
        .collect()
}
