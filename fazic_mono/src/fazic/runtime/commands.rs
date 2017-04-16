use fazic::runtime::ast::{NodeElement, Value};

pub fn print(mut params: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) {
    let param = params.pop();

    let output = match param {
        Some(NodeElement::Value(Value::String(s))) => format!("{}", s),
        Some(NodeElement::Value(Value::Integer(i))) => format!("{}", i),
        Some(NodeElement::Value(Value::Float(f))) => format!("{}", f),
        Some(NodeElement::Error(e)) => format!("ERROR: {}", e),
        _ => unreachable!()
    };

    println!("output {}", output);

    fazic.text_buffer.insert_line(&output);
}
