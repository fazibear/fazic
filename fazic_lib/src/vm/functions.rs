use enums::Value;
use std::char;

use rand::Rng;

pub fn rnd(max: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(max) {
        &Value::Number(max) if max == 0.0 => Value::Number(0.0),
        &Value::Number(max) if max > 0.0 => {
            let rng = fazic.rng.gen_range(0, max as i32) as f64;
            Value::Number(rng)
        }
        &Value::Number(max) if max < 0.0 => {
            let rng = fazic.rng.gen_range(max as i32, 0) as f64;
            Value::Number(rng)
        }
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn abs(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.abs()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sin(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.sin()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn cos(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.cos()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn tan(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.tan()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn atn(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.atan()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn exp(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.exp()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn log(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.ln()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sqr(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.sqrt()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sgn(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) if l == 0.0 => Value::Number(0.0),
        &Value::Number(l) => Value::Number(if l < 0.0 { -1.0 } else { 1.0 }),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn len(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(l) => Value::Number(l.len() as f64),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn asc(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(ref l) => Value::Number(l.chars().nth(0).unwrap() as u32 as f64),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn chr(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => match char::from_u32(l as u32) {
            Some(char) => Value::String(char.to_string()),
            _ => {
                ::vm::error(fazic, "TYPE MISMATCH");
                Value::Null
            }
        },
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn val(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a).clone() {
        Value::String(l) => match l.parse::<f64>() {
            Ok(f) => Value::Number(f),
            _ => {
                ::vm::error(fazic, "TYPE MISMATCH");
                Value::Null
            }
        },
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn int(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.round() as f64),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn str(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::String(l.to_string()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn time(dst: usize, fazic: &mut ::Fazic) {
    let elapsed = fazic.instant.elapsed();
    fazic.variables.set(
        dst,
        Value::Number(elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9),
    );
}
