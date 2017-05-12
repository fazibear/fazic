use fazic::runtime::ast::*;
use fazic::runtime::stack::*;

pub fn print(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>) {
    let output = match params[0] {
        NodeElement::Value(Value::String(ref s)) => format!("{}", s),
        NodeElement::Value(Value::Integer(ref i)) => format!("{}", i),
        NodeElement::Value(Value::Float(ref f)) => format!("{}", f),
        NodeElement::Error(ref e) => format!("ERROR: {}", e),
        _ => unreachable!()
    };

    fazic.text_buffer.insert_line(&output);
    fazic.program.next();
}

pub fn let_(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>) {
    let name = match params[0] {
        NodeElement::Value(Value::String(ref name)) => name.to_string(),
        _ => unreachable!(),
    };

    fazic.program.variables.insert(name.to_string(), params[1].clone());
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

pub fn end(fazic: &mut ::fazic::Fazic){
    fazic.program.stop();
}

pub fn goto(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    match params[0] {
        NodeElement::Value(Value::Integer(ln)) => {
            fazic.program.position = (ln as u16, 0)
        },
        _ => println!("Error in goto")
    }
}

pub fn gosub(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    match params[0] {
        NodeElement::Value(Value::Integer(ln)) => {
            fazic.program.stack.push(StackEntry::Return(fazic.program.position));
            println!("stack: {:?}", fazic.program.stack);
            fazic.program.position = (ln as u16, 0);
        },
        _ => println!("Error in gosub")
    }
}

pub fn return_(fazic: &mut ::fazic::Fazic){
    loop {
        if fazic.program.stack.len() == 0 {
            break;
        }
        match fazic.program.stack.pop() {
            Some(StackEntry::Return(position)) => {
                println!("{:?}", position);
                fazic.program.position = position;
                fazic.program.next();
                break;
            },
            _ => println!("Error in return")

        }
    }
}
