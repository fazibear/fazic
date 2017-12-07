use fazic::enums::Value;
use std::ops::Neg;

pub fn and(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Integer(l & r),
        (&Value::Bool(l), &Value::Bool(r)) => Value::Bool(l && r),
        // (&Value::Float(l), &Value::Float(r)) => Value::Float(l & r),
        // (&Value::Float(l), &Value::Integer(r)) => Value::Float(l & r as f64),
        // (&Value::Integer(l), &Value::Float(r)) => Value::Float(l as f64 & r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn or(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Integer(l | r),
        (&Value::Bool(l), &Value::Bool(r)) => Value::Bool(l || r),
        // (&Value::Float(l), &Value::Float(r)) => Value::Float(l + r),
        // (&Value::Float(l), &Value::Integer(r)) => Value::Float(l + r as f64),
        // (&Value::Integer(l), &Value::Float(r)) => Value::Float(l as f64 + r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}


pub fn add(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Integer(l + r),
        (&Value::Float(l), &Value::Float(r)) => Value::Float(l + r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Float(l + r as f64),
        (&Value::Integer(l), &Value::Float(r)) => Value::Float(l as f64 + r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn eq(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l == r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l == r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l == (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) == r),
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn gt(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l > r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l > r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l > (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) > r),
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn lt(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l < r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l < r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l < (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) < r),
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}


pub fn lteq(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l <= r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l <= r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l <= (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) <= r),
        (_, _) => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn not(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Integer(!l),
        &Value::Bool(l) => Value::Bool(!l),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn neg(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Integer(l.neg()),
        &Value::Float(l) => Value::Float(l.neg()),
        _ => {
            ::fazic::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
