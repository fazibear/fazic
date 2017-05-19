use fazic::runtime::ast::*;
use fazic::runtime::stack::*;
use std::collections::HashMap;

pub fn print(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>) {
    let output = match params[0] {
        NodeElement::Value(Value::String(ref s)) => format!("{}", s),
        NodeElement::Value(Value::Integer(i)) => format!("{}", i),
        NodeElement::Value(Value::Float(f)) => format!("{}", f),
        NodeElement::Value(Value::Bool(b)) => format!("{}", b),
        NodeElement::Error(ref e) => format!("ERROR: {}", e),
        _ => unreachable!("print output param don't match")
    };

    fazic.text_buffer.insert_line(&output);
    fazic.program.next();
}

pub fn let_(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>) {
    let name = match params[0] {
        NodeElement::Value(Value::String(ref name)) => name.to_string(),
        _ => unreachable!("let name params don't match"),
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

pub fn new(fazic: &mut ::fazic::Fazic){
    clr(fazic);
    fazic.program.ast = vec![];
}

pub fn clr(fazic: &mut ::fazic::Fazic){
    fazic.program.stack = vec![];
    fazic.program.variables = HashMap::new();
}

pub fn run(fazic: &mut ::fazic::Fazic){
    clr(fazic);
    cont(fazic);
}

pub fn cont(fazic: &mut ::fazic::Fazic){
    fazic.program.start();
}

pub fn end(fazic: &mut ::fazic::Fazic){
    fazic.program.stop();
}

pub fn goto(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    let line_no = match params[0] {
        NodeElement::Value(Value::Integer(ln)) => ln,
        _ => unreachable!("goto line_no param don't match"),
    };

    if fazic.program.ast[line_no as usize].len() > 0 {
        fazic.program.position = (line_no as u16, 0)
    } else {
        fazic.text_buffer.insert_line("?LINE NOT EXISTS");
        fazic.program.stop();
    };
}

pub fn gosub(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    let line_no = match params[0] {
        NodeElement::Value(Value::Integer(ln)) => ln,
        _ => unreachable!("gosub line_no don't match"),
    };

    if fazic.program.ast[line_no as usize].len() > 0 {
        fazic.program.stack.push(StackEntry::Return(fazic.program.position));
        fazic.program.position = (line_no as u16, 0);
    } else {
        fazic.text_buffer.insert_line("?LINE NOT EXISTS");
        fazic.program.stop();
    };
}

pub fn return_(fazic: &mut ::fazic::Fazic){
    loop {
        if fazic.program.stack.len() == 0 {
            fazic.text_buffer.insert_line("?RETURN WITHOUT GOSUB");
            fazic.program.stop();
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

pub fn for_(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    let var_name = match params[0] {
        NodeElement::Value(Value::String(ref name)) => name.to_string(),
        _ => unreachable!("for var name don't match"),
    };

    let init_val = match params[1] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!("for init_val don't match"),
    };

    let max_val = match params[2] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!("for max_val don't match"),
    };

    let step_val = match params[3] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!("for step_val don't match"),
    };

    fazic.program.variables.insert(var_name.clone(), NodeElement::Value(Value::Float(init_val)));
    fazic.program.stack.push(StackEntry::Next(fazic.program.position, var_name.clone(), max_val, step_val));
    fazic.program.next();
}

pub fn next(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){

    let next_var = match params.get(0) {
        Some(&NodeElement::Value(Value::String(ref name))) => Some(name),
        _ => None,
    };

    loop {
        if fazic.program.stack.len() == 0 {
            fazic.text_buffer.insert_line("?NEXT WITHOUT FOR");
            fazic.program.stop();
            break;
        }

        let (position, var_name, max_val, step_val) = match (next_var, fazic.program.stack.last().cloned()) {
            (None, Some(StackEntry::Next(position, ref var_name, max_val, step_val))) =>
                (position, var_name.clone(), max_val, step_val),
            (Some(next_var), Some(StackEntry::Next(position, ref var_name, max_val, step_val))) if &var_name == &next_var =>
                (position, var_name.clone(), max_val, step_val),
            _ => {
                fazic.program.stack.pop();
                continue;
            }
        };

        let var = match fazic.program.variables.get(&var_name) {
            Some(&NodeElement::Value(Value::Float(var))) => var,
            _ => unreachable!("next var name don't match"),
        };


        if var < max_val {
            fazic.program.variables.insert(var_name.clone(), NodeElement::Value(Value::Float(var + step_val)));
            fazic.program.position = position;
        } else {
            fazic.program.stack.pop();
        }

        fazic.program.next();
        break;
    }
}

pub fn if_(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    let expression = match params[0] {
        NodeElement::Value(Value::Bool(b)) => b,
        _ => unreachable!("if expression don't match"),
    };

    if !expression {
        let line_no = fazic.program.position.0;
        let col_no = fazic.program.ast[line_no as usize].len() as u16 - 1;
        fazic.program.position = (line_no, col_no);
    }
    fazic.program.next();
}
