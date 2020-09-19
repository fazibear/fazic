use vm::Instruction;
use vm::Value;

pub fn push(fazic: &mut ::Fazic) {
    let val = match *fazic.vm.current() {
        Instruction::Push(ref stack) => stack.clone(),
        _ => unreachable!(),
    };
    fazic.stack.push(val);
}

pub fn mov(to: usize, from: usize, fazic: &mut ::Fazic) {
    let val = fazic.variables.get(from).clone();
    fazic.variables.set(to, val);
}

pub fn set_var(name: usize, fazic: &mut ::Fazic) {
    let val = match *fazic.vm.current() {
        Instruction::Set(_, ref val) => val,
        _ => unreachable!(),
    };
    fazic.variables.set(name, val.clone());
}

pub fn jmpif(var: usize, pos: usize, fazic: &mut ::Fazic) {
    let cond = match *fazic.variables.get(var) {
        //Value::Integer(_) | Value::Float(_) => true,
        Value::Bool(b) => b,
        _ => false,
    };

    if cond {
        fazic.vm.jump(pos)
    } else {
        fazic.vm.step()
    };
}

pub fn jmpifnot(var: usize, pos: usize, fazic: &mut ::Fazic) {
    let cond = match *fazic.variables.get(var) {
        //Value::Integer(_) | Value::Float(_) => true,
        Value::Bool(b) => b,
        _ => false,
    };

    if cond {
        fazic.vm.step()
    } else {
        fazic.vm.jump(pos)
    };
}
