use fazic::enums::*;
use fazic::nodes::Node;
use fazic::variables::Variables;
use fazic::lines::Lines;

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
    lines: &mut Lines,
    dst: usize,
    nest: usize,
) {
    let params = process_nodes(instructions, nodes, variables, lines, nest);
    println!("{}: {:?}", name, params);

    match name {
        "run" => instructions.push(Instruction::Run),
        "list" => instructions.push(Instruction::List),
        "new" => instructions.push(Instruction::New),
        "clr" => instructions.push(Instruction::Clr),
        "flip" => instructions.push(Instruction::Flip),
        "goto" => if let Param::Value(Value::Integer(i)) = params[0] {
            instructions.push(Instruction::JmpLine(i as u16));
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
        "and" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::And(p0, p1, dst));
        }
        "or" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Or(p0, p1, dst));
        }
        "add" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Add(p0, p1, dst));
        }
        "eq" => {
            let p0 = process_param(0, &params, instructions);
            let p1 = process_param(1, &params, instructions);
            instructions.push(Instruction::Eq(p0, p1, dst));
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
        "rng" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Rng(p0, dst));
        }
        "abs" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Abs(p0, dst));
        }
        "sin" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Sin(p0, dst));
        }
        "cos" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Cos(p0, dst));
        }
        "tan" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Tan(p0, dst));
        }
        "atn" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Atn(p0, dst));
        }
        "exp" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Exp(p0, dst));
        }
        "log" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Log(p0, dst));
        }
        "sqr" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Sqr(p0, dst));
        }
        "sgn" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Sgn(p0, dst));
        }
        "len" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Len(p0, dst));
        }
        "chr" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Chr(p0, dst));
        }
        "asc" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Asc(p0, dst));
        }
        "val" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Val(p0, dst));
        }
        "int" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Int(p0, dst));
        }
        "str" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Str(p0, dst));
        }
        "neg" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Neg(p0, dst));
        }
        "not" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::Not(p0, dst));
        }
        "gosub" => if let Param::Value(Value::Integer(i)) = params[0] {
            let jmp = instructions.len() + 2;
            instructions.push(Instruction::Push(Stack::Return(jmp)));
            instructions.push(Instruction::JmpLine(i as u16));
        },
        "return" => instructions.push(Instruction::Return),
        "end" => instructions.push(Instruction::End),
        "stop" => instructions.push(Instruction::Stop),
        "cont" => instructions.push(Instruction::Cont),
        "if" => {
            let p0 = process_param(0, &params, instructions);
            instructions.push(Instruction::JmpIfNotNextLine(p0, lines.current()));
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
    lines: &mut Lines,
    nest: usize,
) -> Vec<Param> {
    let mut params: Vec<Param> = vec![];
    for (i, node) in nodes.iter().enumerate() {
        let tmp = variables.alloc(&format!("{}-{}-TMP", nest, i));
        match *node {
            NodeElement::Node(Node(ref str, ref nodes)) => {
                process_node(instructions, str, nodes, variables, lines, tmp, nest+1);
                params.push(Param::Node(tmp));
            }
            NodeElement::Value(ref val) => {
                params.push(Param::Value(val.clone()));
            }
            NodeElement::Var(ref name) => {
                params.push(Param::Variable(variables.alloc(name)));
            }
            NodeElement::LineNo(line) => {
                lines.add(line, instructions.len());
            }
        }
    }
    params
}

pub fn process_gotos(instruction: Instruction, lines: &Lines) -> Instruction {
    match instruction {
        Instruction::JmpLine(ref line) => match lines.get(line) {
            Some(pos) => Instruction::Jmp(*pos as usize),
            None => Instruction::Error("UNDEF'D STATEMENT ERROR".to_string()),
        },
        Instruction::JmpIfNotNextLine(var, ref line) => match lines.get_next(line) {
            Some(pos) => Instruction::JmpIfNot(var, *pos as usize),
            None => Instruction::Error("UNDEF'D STATEMENT ERROR".to_string()),
        },
        _ => instruction,
    }
}

pub fn compile(
    nodes: &[NodeElement],
    variables: &mut Variables,
    mut lines: &mut Lines,
) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    println!("nodes: {:?}", nodes);

    lines.reset();

    process_nodes(&mut instructions, nodes, variables, &mut lines, 0);

    let end_line = lines.current() + 1;
    lines.add(end_line, instructions.len());
    instructions.push(Instruction::End);

    println!("instructions: {:?}", instructions);

    let x = instructions
        .into_iter()
        .map(|i| process_gotos(i, lines))
        .collect();

    println!("instructions: {:?}", x);
    x
}
