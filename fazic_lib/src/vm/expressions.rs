use enums::Value;
use std::ops::Neg;

pub fn and(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number((l as i64 & r as i64) as f64),
        (&Value::Bool(l), &Value::Bool(r)) => Value::Bool(l && r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn or(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number((l as i64 | r as i64) as f64),
        (&Value::Bool(l), &Value::Bool(r)) => Value::Bool(l || r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn add(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l + r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn sub(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l - r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn mul(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l * r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
pub fn div(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (_, &Value::Number(r)) if r == 0.0 => {
            ::vm::error(fazic, "DIVISION BY ZERO");
            Value::Null
        }
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l / r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn mod_(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l % r),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn pow(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Number(l.powf(r)),
        // (&Value::String(ref l), &Value::String(ref r)) => {
        //     let mut str = l.clone();
        //     str.push_str(r);
        //     Value::String(str)
        // }
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn eq(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Bool(l == r),
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn gt(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Bool(l > r),
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn lt(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Bool(l < r),
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn lteq(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Bool(l <= r),
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn gteq(a: usize, b: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match (fazic.variables.get(a), fazic.variables.get(b)) {
        (&Value::Number(l), &Value::Number(r)) => Value::Bool(l >= r),
        (_, _) => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn not(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(!(l as i64) as f64),
        &Value::Bool(l) => Value::Bool(!l),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}

pub fn neg(a: usize, dst: usize, fazic: &mut ::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Number(l) => Value::Number(l.neg()),
        _ => {
            ::vm::error(fazic, "TYPE MISMATCH");
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}
