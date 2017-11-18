use fazic::enums::Value;

pub fn abs(a: usize, dst: usize, fazic: &mut ::fazic::Fazic) {
    let ret = match fazic.variables.get(a) {
        &Value::Integer(l) => Value::Integer(l.abs()),
        &Value::Float(l) => Value::Float(l.abs()),
        _ => {
            // error("TYPE MISMATCH".to_string(), fazic);
            Value::Null
        }
    };
    fazic.variables.set(dst, ret);
}