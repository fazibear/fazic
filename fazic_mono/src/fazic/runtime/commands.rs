use fazic::runtime::ast::{NodeElement, Value};

pub fn print(mut params: Vec<NodeElement>) {
    let param = params.pop();

    match param {
        Some(NodeElement::Value(Value::String(s))) => println!("print: {}", s),
        Some(NodeElement::Value(Value::Integer(i))) => println!("print: {}", i),
        Some(NodeElement::Value(Value::Float(f))) => println!("print: {}", f),
        Some(NodeElement::Error(e)) => println!("ERROR: {}", e),
        _ => unreachable!()
    }
}
