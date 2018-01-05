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
    fazic.variables.set(dst, Value::Number(0.0));
}

pub fn abs(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.abs()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sin(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.sin()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn cos(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.cos()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn tan(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.tan()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn atn(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.atan()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn exp(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.exp()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn log(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.ln()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sqr(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.sqrt()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sgn(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) if l == 0.0 => Value::Number(0.0),
        &Value::Number(l) => Value::Number(if l < 0.0 { -1.0 } else { 1.0 }),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn len(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(l) => Value::Number(l.len() as f64),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn asc(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(ref l) => Value::Number(l.chars().nth(0).unwrap() as u32 as f64),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn chr(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => match char::from_u32(l as u32) {
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
            Ok(f) => Value::Number(f),
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
        &Value::Number(l) => Value::Number(l.round() as f64),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn str(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::String(l.to_string()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
