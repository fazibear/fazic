use fazic::vm::Value;
use rand::SeedableRng;

pub fn print(var: usize, fazic: &mut ::fazic::Fazic) {
    let string = match *fazic.variables.get(var) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        Value::Number(n) if n < 1_000_000_000.0 => format!("{}", n),
        Value::Number(n) => format!("{:E}", n),
        Value::Bool(b) => format!("{}", b),
        Value::Null => "null".to_string(),
    };

    fazic.text_buffer.insert_line(&string);
}

pub fn color(var: usize, fazic: &mut ::fazic::Fazic) {
    let color = match *fazic.variables.get(var) {
        Value::Number(i) => i as u8,
        _ => 0,
    };
    fazic.screen.current_color = color;
    fazic.text_buffer.current_color = color;
}

pub fn clear(var: usize, fazic: &mut ::fazic::Fazic) {
    let color = match *fazic.variables.get(var) {
        Value::Number(i) => i as u8,
        _ => 0,
    };
    fazic.screen.clear(color);
}

pub fn srand(var: usize, fazic: &mut ::fazic::Fazic) {
    let val = match *fazic.variables.get(var) {
        Value::Number(i) => i as u32,
        _ => 0,
    };
    fazic.rng = SeedableRng::from_seed([val, val+1, val+2, val+3])
}


pub fn dot(x: usize, y: usize, fazic: &mut ::fazic::Fazic) {
    let x = match *fazic.variables.get(x) {
        Value::Number(x) => x as u16,
        _ => 0,
    };

    let y = match *fazic.variables.get(y) {
        Value::Number(y) => y as u16,
        _ => 0,
    };

    let color = fazic.screen.current_color;

    fazic.screen.put_pixel(x, y, color);
}

pub fn line(x: usize, y: usize, x2: usize, y2: usize, fazic: &mut ::fazic::Fazic) {
    let color = fazic.screen.current_color;
    let x = match *fazic.variables.get(x) {
        Value::Number(x) => x as u16,
        _ => 0,
    };
    let y = match *fazic.variables.get(y) {
        Value::Number(y) => y as u16,
        _ => 0,
    };
    let x2 = match *fazic.variables.get(x2) {
        Value::Number(x2) => x2 as u16,
        _ => 0,
    };
    let y2 = match *fazic.variables.get(y2) {
        Value::Number(y2) => y2 as u16,
        _ => 0,
    };

    fazic.screen.line(x, y, x2, y2, color);
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
