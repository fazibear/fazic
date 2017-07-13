use ::fazic::vm::Instruction;
use ::fazic::vm::Value;

pub fn push(fazic: &mut ::fazic::Fazic) {
    let val = match *fazic.vm.current() {
        Instruction::Push(ref stack) => stack.clone(),
        _ => unreachable!(),
    };
    fazic.stack.push(val);
}

pub fn set_var(name: usize, fazic: &mut ::fazic::Fazic) {
    fazic.variables[name] = match *fazic.vm.current() {
        Instruction::SetVar(_, ref val) => val.clone(),
        _ => unreachable!(),
    };
}

pub fn jmp(pos: usize, fazic: &mut ::fazic::Fazic) {
    fazic.vm.position = pos;
}

pub fn jmpif(pos: usize, var: usize, fazic: &mut ::fazic::Fazic) {
    let cond = match fazic.variables[var] {
        Value::Integer(_) | Value::Float(_) => true,
        Value::Bool(b) => b,
        _ => false,
    };

    if cond {
        fazic.vm.position = pos
    } else {
        fazic.vm.position += 1
    };
}
