use ::fazic::enums::*;
use ::fazic::nodes::Node;
use ::fazic::variables::Variables;

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

fn process_node(instructions: &mut Vec<Instruction>, name: &str, nodes: &[NodeElement], variables: &mut Variables, ii: usize) {
    let params = process_nodes(instructions, nodes, variables);
    println!("{}: {:?}", name, params);

    match name {
        "run"     => instructions.push(Instruction::Run),
        "list"    => instructions.push(Instruction::List),
        "new"    => instructions.push(Instruction::New),
        "clr"    => instructions.push(Instruction::Clr),
        "load"   => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Load(p0));
        },
        "save"   => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Save(p0));
        },
        "color"   => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Color(p0));
        },
        "print"   => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Print(p0));
        },
        "add"     => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Add(p0, p1, ii));
        }
        "lt"      => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Lt(p0, p1, ii));
        }
        "gt"      => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Gt(p0, p1, ii));
        }
        "lteq"    => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::LtEq(p0, p1, ii));
        }
        "let"     => {
            let p0 = process_param(0, &params, instructions);
            match params[1] {
                Param::Value(ref val) => instructions.push(Instruction::Set(p0, val.clone())),
                Param::Node(i) | Param::Variable(i) => instructions.push(Instruction::Mov(p0, i)),
            }
        }
        "next"    => {
            instructions.push(Instruction::Next);
            instructions.push(Instruction::Pop);
        },
        "for"     => {
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
        },
        _ => {
            println!("Can't translate: {}", name);
        }
    }
}

fn process_nodes(instructions: &mut Vec<Instruction>, nodes: &[NodeElement], variables: &mut Variables) -> Vec<Param> {
    let mut params: Vec<Param> = vec![];
    for (i, node) in nodes.iter().enumerate() {
        let tmp = variables.alloc(&format!("{}-TMP", i));
        match *node {
            NodeElement::Node(Node(ref str, ref nodes)) => {
                process_node(instructions, str, nodes, variables, tmp);
                params.push(Param::Node(tmp));
            },
            NodeElement::Value(ref val) => {
                params.push(Param::Value(val.clone()));
            },
            NodeElement::Var(ref name) => {
                params.push(Param::Variable(variables.alloc(name)));
            }
        }
    };
    params
}

pub fn compile(nodes: &[NodeElement], variables: &mut Variables) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    process_nodes(&mut instructions, nodes, variables);

    instructions.push(Instruction::Stop);

    println!("instructions: {:?}", instructions);

    instructions
}
