use fazic::vm::Value;

pub fn print(var: usize, fazic: &mut ::fazic::Fazic) {
    let string = match *fazic.variables.get(var) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        Value::Integer(i) => format!("{}", i),
        Value::Float(f) => format!("{:.4}", f),
        Value::Bool(b) => format!("{}", b),
        Value::Null => "null".to_string(),
    };

    fazic.text_buffer.insert_line(&string);
}

pub fn color(var: usize, fazic: &mut ::fazic::Fazic) {
    let color = match *fazic.variables.get(var) {
        Value::Integer(i) => i as u8,
        Value::Float(f) => f as u8,
        _ => 0,
    };
    fazic.screen.current_color = color;
    fazic.text_buffer.current_color = color;
}

pub fn dot(x: usize, y: usize, fazic: &mut ::fazic::Fazic) {
    let x = match *fazic.variables.get(x) {
        Value::Integer(x) => x as u16,
        Value::Float(x) => x as u16,
        _ => 0,
    };

    let y = match *fazic.variables.get(y) {
        Value::Integer(y) => y as u16,
        Value::Float(y) => y as u16,
        _ => 0,
    };

    let color = fazic.screen.current_color;
    //    println!("{}, {}", x, y);

    fazic.screen.put_pixel(x, y, color);
}

pub fn flip(fazic: &mut ::fazic::Fazic) {
    fazic.redraw = true;
}

pub fn mode(mode: u8, fazic: &mut ::fazic::Fazic) {
    fazic.mode = mode;
}

pub fn list(fazic: &mut ::fazic::Fazic) {
    for &(_, ref string, _) in &fazic.program.lines {
        fazic.text_buffer.insert_line(string);
    }
}

pub fn dir(fazic: &mut ::fazic::Fazic) {
    for line in ::targets::dir() {
        fazic.text_buffer.insert_line(&line);
    }
}

pub fn load(name: usize, fazic: &mut ::fazic::Fazic) {
    let name = match *fazic.variables.get(name) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        _ => "".to_string(),
    };

    fazic.program = ::fazic::program::Program::new();

    match ::targets::load(&name) {
        Ok(resp) => {
            // new(fazic);

            for line in resp.lines() {
                ::fazic::parse(fazic, line);
            }

            fazic.text_buffer.insert_line("LOADED");
        }
        Err(resp) => {
            let msg = format!("? {}", resp.to_uppercase());
            fazic.text_buffer.insert_line(&msg);
        }
    }
}

pub fn save(name: usize, fazic: &mut ::fazic::Fazic) {
    let name = match *fazic.variables.get(name) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        _ => "".to_string(),
    };

    let mut program = String::new();

    for &(_, ref string, _) in &fazic.program.lines {
        program.push_str(string);
        program.push_str("\n");
    }

    match ::targets::save(&name, &program) {
        Ok(resp) => {
            fazic.text_buffer.insert_line(&resp);
        }
        Err(resp) => {
            let msg = format!("? {}", resp.to_uppercase());
            fazic.text_buffer.insert_line(&msg);
        }
    }
}
