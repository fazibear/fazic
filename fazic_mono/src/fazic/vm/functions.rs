use fazic::enums::Value;

pub fn rng(_: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    // let ret = match fazic.variables.get(a) {
    //     &Value::Integer(l) => Value::Integer(l.abs()),
    //     &Value::Float(l) => Value::Float(l.abs()),
    //     _ => {
    //         ::fazic::vm::error(fazic, "TYPE MISMATCH");
    //         Value::Null
    //     }
    // };
    fazic.variables.set(dst, Value::Integer(0));
}

pub fn abs(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Integer(l.abs()),
        &Value::Float(l) => Value::Float(l.abs()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sin(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).sin()),
        &Value::Float(l) => Value::Float(l.sin()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn cos(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).cos()),
        &Value::Float(l) => Value::Float(l.cos()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn tan(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).tan()),
        &Value::Float(l) => Value::Float(l.tan()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
