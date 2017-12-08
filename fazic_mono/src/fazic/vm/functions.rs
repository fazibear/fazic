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

pub fn atn(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).atan()),
        &Value::Float(l) => Value::Float(l.atan()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn exp(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).exp()),
        &Value::Float(l) => Value::Float(l.exp()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn log(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).ln()),
        &Value::Float(l) => Value::Float(l.ln()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sqr(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Float((l as f64).sqrt()),
        &Value::Float(l) => Value::Float(l.sqrt()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sgn(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(0) => Value::Integer(0),
        &Value::Float(0.0) => Value::Integer(0),
        &Value::Integer(l) => Value::Integer(if l < 0 { -1 } else { 1 }),
        &Value::Float(l) => Value::Integer(if l < 0.0 { -1 } else { 1 }),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
