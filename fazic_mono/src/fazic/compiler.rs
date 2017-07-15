use ::fazic::enums::*;
use ::fazic::nodes::Node;

fn process_node(instructions: &mut Vec<Instruction>, name: &str, params: &[NodeElement]) {
    let nums = process_nodes(instructions, params);
    println!("{}: {:?}", name, nums);

    match name {
        "run"     => instructions.push(Instruction::Run),
        "print"   => instructions.push(Instruction::Print(nums[0])),
        "add"     => instructions.push(Instruction::Add(nums[0],nums[1], 0)),
        "lt"      => instructions.push(Instruction::Lt(nums[0],nums[1], 0)),
        "gt"      => instructions.push(Instruction::Gt(nums[0],nums[1], 0)),
        "lteq"    => instructions.push(Instruction::LtEq(nums[0],nums[1], 0)),
        _ => {
            println!("Can't translate: {}", name);
        }
    }
}

fn process_nodes(instructions: &mut Vec<Instruction>, nodes: &[NodeElement]) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    for (i, node) in nodes.iter().enumerate() {
        match *node {
            NodeElement::Node(Node(ref str, ref params)) => {
                process_node(instructions, str, params);
                nums.push(0);
            },
            NodeElement::Value(ref val) => {
                instructions.push(Instruction::SetVar(i, val.clone()));
                nums.push(i);
            },
            NodeElement::Var(ref name) => {
                nums.push(10);
            }
        }
    };
    nums
}

pub fn compile(nodes: &[NodeElement]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    process_nodes(&mut instructions, nodes);

    instructions.push(Instruction::Stop);

    println!("{:?}", instructions);

    instructions
}
