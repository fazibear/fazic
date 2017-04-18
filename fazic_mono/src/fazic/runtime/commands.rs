use fazic::runtime::ast::{NodeElement, Value};

pub fn print(fazic: &mut ::fazic::Fazic, mut params: Vec<NodeElement>) {
    let param = params.pop();

    let output = match param {
        Some(NodeElement::Value(Value::String(s))) => format!("{}", s),
        Some(NodeElement::Value(Value::Integer(i))) => format!("{}", i),
        Some(NodeElement::Value(Value::Float(f))) => format!("{}", f),
        Some(NodeElement::Error(e)) => format!("ERROR: {}", e),
        _ => unreachable!()
    };

    fazic.text_buffer.insert_line(&output);
}

pub fn list(fazic: &mut ::fazic::Fazic){
    for &(_, ref string) in &fazic.program.lines {
        fazic.text_buffer.insert_line(&string);
    }
}
