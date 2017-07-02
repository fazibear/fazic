use ::fazic::vm::Instruction;
use ::fazic::vm::Value;

pub fn set_var(name: usize, fazic: &mut ::fazic::Fazic) {
    fazic.vm.variables[name] = match fazic.vm.current() {
        &Instruction::SetVar(_, ref val) => val.clone(),
        _ => unreachable!(),
    };
}

pub fn jmp(pos: usize, fazic: &mut ::fazic::Fazic) {
    fazic.vm.position = pos;
}

pub fn jmpif(pos: usize, var: usize, fazic: &mut ::fazic::Fazic) {
    let cond = match &fazic.vm.variables[var] {
        &Value::String(_) => false,
        &Value::Integer(i) if i == 0 => false,
        &Value::Integer(_) => true,
        &Value::Float(f) if f == 0.0 => false,
        &Value::Float(_) => true,
        &Value::Bool(b) => b,
        &Value::Null => false,
    };

    if cond {
        fazic.vm.position = pos
    } else {
        fazic.vm.position += 1
    };
}
