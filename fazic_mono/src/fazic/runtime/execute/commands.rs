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
    let line_no = match params[0] {
        NodeElement::Value(Value::Integer(ln)) => ln,
        _ => unreachable!(),
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
        _ => unreachable!(),
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
        _ => unreachable!(),
    };

    let init_val = match params[1] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!(),
    };

    let max_val = match params[2] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!(),
    };

    let step_val = match params[3] {
        NodeElement::Value(Value::Integer(i)) => i as f64,
        NodeElement::Value(Value::Float(f)) => f,
        _ => unreachable!(),
    };

    fazic.program.variables.insert(var_name.clone(), NodeElement::Value(Value::Float(init_val)));
    fazic.program.stack.push(StackEntry::Next(fazic.program.position, var_name.clone(), max_val, step_val));
    fazic.program.next();
}

pub fn next(fazic: &mut ::fazic::Fazic, params: Vec<NodeElement>){
    loop {
        if fazic.program.stack.len() == 0 {
            fazic.text_buffer.insert_line("?NEXT WITHOUT FOR");
            fazic.program.stop();
            break;
        }

        let (position, var_name, max_val, step_val) = match fazic.program.stack.last().cloned() {
            Some(StackEntry::Next(position, ref var_name, max_val, step_val)) => (position, var_name.clone(), max_val, step_val),
            _ => {
                println!("pop");
                fazic.program.stack.pop();
                continue;
            }
        };

        let var = match fazic.program.variables.get(&var_name) {
            Some(&NodeElement::Value(Value::Float(var))) => var,
            _ => unreachable!(),
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
