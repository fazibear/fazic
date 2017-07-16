use ::fazic::enums::*;
use ::fazic::nodes::Node;
use ::fazic::variables::Variables;

fn process_node(instructions: &mut Vec<Instruction>, name: &str, params: &[NodeElement], variables: &mut Variables) {
    let nums = process_nodes(instructions, params, variables);
    println!("{}: {:?}", name, nums);

    match name {
        "run"     => instructions.push(Instruction::Run),
        "print"   => instructions.push(Instruction::Print(nums[0])),
        "add"     => instructions.push(Instruction::Add(nums[0],nums[1], 0)),
        "lt"      => instructions.push(Instruction::Lt(nums[0],nums[1], 0)),
        "gt"      => instructions.push(Instruction::Gt(nums[0],nums[1], 0)),
        "lteq"    => instructions.push(Instruction::LtEq(nums[0],nums[1], 0)),
        "let"     => instructions.push(Instruction::Mov(nums[0], nums[1])),
        "next"    => {
            instructions.push(Instruction::Next);
            instructions.push(Instruction::Pop);
        },
        "for"     => {
            let max = variables.alloc(&format!("{}-MAX", nums[0]));
            let step = variables.alloc(&format!("{}-STEP", nums[0]));
            instructions.push(Instruction::Mov(nums[0],nums[1]));
            instructions.push(Instruction::Mov(max,nums[2]));
            instructions.push(Instruction::Mov(step,nums[3]));
            let jmp = instructions.len() + 1;
            instructions.push(Instruction::Push(Stack::Next(nums[0], max, step, jmp)));
        },
        _ => {
            println!("Can't translate: {}", name);
        }
    }
}

fn process_nodes(instructions: &mut Vec<Instruction>, nodes: &[NodeElement], variables: &mut Variables) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    for (i, node) in nodes.iter().enumerate() {
        match *node {
            NodeElement::Node(Node(ref str, ref params)) => {
                process_node(instructions, str, params, variables);
                nums.push(0);
            },
            NodeElement::Value(ref val) => {
                instructions.push(Instruction::Set(i, val.clone()));
                nums.push(i);
            },
            NodeElement::Var(ref name) => {
                nums.push(variables.alloc(name));
            }
        }
    };
    nums
}

pub fn compile(nodes: &[NodeElement], variables: &mut Variables) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    process_nodes(&mut instructions, nodes, variables);

    instructions.push(Instruction::Stop);

    println!("{:?}", instructions);

    instructions
}
