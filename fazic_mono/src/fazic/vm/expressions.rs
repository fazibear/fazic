use ::fazic::enums::Value;

pub fn add(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (&fazic.variables[a], &fazic.variables[b]) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Integer(l + r),
        (&Value::Float(l), &Value::Float(r)) => Value::Float(l + r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Float(l + r as f64),
        (&Value::Integer(l), &Value::Float(r)) => Value::Float(l as f64 + r),
        (&Value::String(ref l), &Value::String(ref r)) => {
            let mut str = l.clone();
            str.push_str(r);
            Value::String(str)
        },
        (_, _) => {
            // error("TYPE MISMATCH".to_string(), fazic);
            Value::Null
        },
    };
    fazic.variables[dst] = ret;
}
pub fn gt(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (&fazic.variables[a], &fazic.variables[b]) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l > r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l > r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l > (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) > r),
        (_, _) => {
            // error("TYPE MISMATCH".to_string(), fazic);
            Value::Null
        },
    };
    fazic.variables[dst] = ret;
}

pub fn lt(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (&fazic.variables[a], &fazic.variables[b]) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l < r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l < r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l < (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) < r),
        (_, _) => {
            // error("TYPE MISMATCH".to_string(), fazic);
            Value::Null
        },
    };
    fazic.variables[dst] = ret;
}


pub fn lteq(a: usize, b: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match (&fazic.variables[a], &fazic.variables[b]) {
        (&Value::Integer(l), &Value::Integer(r)) => Value::Bool(l <= r),
        (&Value::Float(l), &Value::Float(r)) => Value::Bool(l <= r),
        (&Value::Float(l), &Value::Integer(r)) => Value::Bool(l <= (r as f64)),
        (&Value::Integer(l), &Value::Float(r)) => Value::Bool((l as f64) <= r),
        (_, _) => {
            // error("TYPE MISMATCH".to_string(), fazic);
            Value::Null
        },
    };
    fazic.variables[dst] = ret;
}
