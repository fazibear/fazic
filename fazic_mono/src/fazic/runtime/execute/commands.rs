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
    fazic.program.next();
}

pub fn list(fazic: &mut ::fazic::Fazic){
    for &(_, ref string) in &fazic.program.lines {
        fazic.text_buffer.insert_line(&string);
    }
    fazic.program.stop();
}

pub fn run(fazic: &mut ::fazic::Fazic){
    fazic.program.start();
}

pub fn goto(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    match params[0] {
        NodeElement::Value(Value::Integer(ln)) => fazic.program.position = (ln as u16, 0),
        _ => println!("Error")
    }
}
