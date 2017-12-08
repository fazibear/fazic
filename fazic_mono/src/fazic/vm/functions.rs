use fazic::enums::Value;
use std::char;

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
        &Value::Float(l) if l == 0.0 => Value::Integer(0),
        &Value::Integer(l) => Value::Integer(if l < 0 { -1 } else { 1 }),
        &Value::Float(l) => Value::Integer(if l < 0.0 { -1 } else { 1 }),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn len(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(l) => Value::Integer(l.len() as i32),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn asc(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(ref l) => Value::Integer(l.chars().nth(0).unwrap() as i32),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn chr(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => match char::from_u32(l as u32) {
            Some(char) => Value::String(char.to_string()),
            _ => {
                ::fazic::vm::error(fazic, "TYPE MISMATCH");
                Value::Null
            }
        },
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn val(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(l) => match l.parse::<f64>() {
            Ok(f) => Value::Float(f),
            _ => {
                ::fazic::vm::error(fazic, "TYPE MISMATCH");
                Value::Null
            }
        },
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn int(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Float(l) => Value::Integer(l as i32),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn str(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::String(l.to_string()),
        &Value::Float(l) => Value::String(l.to_string()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
