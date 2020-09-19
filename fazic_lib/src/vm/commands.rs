use rand::SeedableRng;
use vm::Value;

pub fn print(var: usize, fazic: &mut ::Fazic) {
    let string = match *fazic.variables.get(var) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        Value::Number(n) if n < 1_000_000_000.0 => format!("{}", n),
        Value::Number(n) => format!("{:E}", n),
        Value::Bool(b) => format!("{}", b),
        Value::Null => "null".to_string(),
    };

    fazic.text_buffer.insert_line(&string);
}

pub fn color(var: usize, fazic: &mut ::Fazic) {
    let color = match *fazic.variables.get(var) {
        Value::Number(i) => i as u8,
        _ => 0,
    };
    fazic.screen.current_color = color;
    fazic.text_buffer.current_color = color;
}

pub fn clear(var: usize, fazic: &mut ::Fazic) {
    let color = match *fazic.variables.get(var) {
        Value::Number(i) => i as u8,
        _ => 0,
    };
    fazic.screen.clear(color);
}

pub fn srand(var: usize, fazic: &mut ::Fazic) {
    let val = match *fazic.variables.get(var) {
        Value::Number(i) => i as u8,
        _ => 0,
    };
    fazic.rng = SeedableRng::from_seed([
        val,
        val + 1,
        val + 2,
        val + 3,
        val + 4,
        val + 5,
        val + 6,
        val + 7,
        val + 8,
        val + 9,
        val + 10,
        val + 11,
        val + 12,
        val + 13,
        val + 14,
        val + 15,
    ])
}

pub fn dot(x: usize, y: usize, fazic: &mut ::Fazic) {
    let x = match *fazic.variables.get(x) {
        Value::Number(x) => x as i32,
        _ => 0,
    };

    let y = match *fazic.variables.get(y) {
        Value::Number(y) => y as i32,
        _ => 0,
    };

    let color = fazic.screen.current_color;

    fazic.screen.put_pixel(x, y, color);
}

pub fn line(x1: usize, y1: usize, x2: usize, y2: usize, fazic: &mut ::Fazic) {
    let color = fazic.screen.current_color;
    let x1 = match *fazic.variables.get(x1) {
        Value::Number(x1) => x1 as i32,
        _ => 0,
    };
    let y1 = match *fazic.variables.get(y1) {
        Value::Number(y1) => y1 as i32,
        _ => 0,
    };
    let x2 = match *fazic.variables.get(x2) {
        Value::Number(x2) => x2 as i32,
        _ => 0,
    };
    let y2 = match *fazic.variables.get(y2) {
        Value::Number(y2) => y2 as i32,
        _ => 0,
    };
    //debug!("LINE: {} {} {} {}", x1,y1,x2,y2);
    fazic.screen.line(x1, y1, x2, y2, color);
}

pub fn circle(x1: usize, y1: usize, r: usize, fazic: &mut ::Fazic) {
    let color = fazic.screen.current_color;
    let x1 = match *fazic.variables.get(x1) {
        Value::Number(x1) => x1 as i32,
        _ => 0,
    };
    let y1 = match *fazic.variables.get(y1) {
        Value::Number(y1) => y1 as i32,
        _ => 0,
    };
    let r = match *fazic.variables.get(r) {
        Value::Number(x2) => x2 as i32,
        _ => 0,
    };

    fazic.screen.circle(x1, y1, r, color);
}

pub fn flip(fazic: &mut ::Fazic) {
    fazic.screen.redraw = true;
}

pub fn mode(mode: u8, fazic: &mut ::Fazic) {
    fazic.mode = mode;
}

pub fn list(fazic: &mut ::Fazic) {
    for &(_, ref string, _) in &fazic.program.lines {
        fazic.text_buffer.insert_line(string);
    }
}

pub fn dir(fazic: &mut ::Fazic) {
    // for line in ::targets::dir() {
    //     fazic.text_buffer.insert_line(&line);
    // }
}

pub fn load(name: usize, fazic: &mut ::Fazic) {
    let name = match *fazic.variables.get(name) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        _ => "".to_string(),
    };

    fazic.program = ::program::Program::new();

    // match ::targets::load(&name) {
    //     Ok(resp) => {
    //         // new(fazic);
    //
    //         for line in resp.lines() {
    //             ::parse(fazic, line);
    //         }
    //
    //         fazic.text_buffer.insert_line("LOADED");
    //     }
    //     Err(resp) => {
    //         let msg = format!("? {}", resp.to_uppercase());
    //         fazic.text_buffer.insert_line(&msg);
    //     }
    // }
}

pub fn save(name: usize, fazic: &mut ::Fazic) {
    let name = match *fazic.variables.get(name) {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        _ => "".to_string(),
    };

    let mut program = String::new();

    for &(_, ref string, _) in &fazic.program.lines {
        program.push_str(string);
        program.push_str("\n");
    }

    // match ::targets::save(&name, &program) {
    //     Ok(resp) => {
    //         fazic.text_buffer.insert_line(&resp);
    //     }
    //     Err(resp) => {
    //         let msg = format!("? {}", resp.to_uppercase());
    //         fazic.text_buffer.insert_line(&msg);
    //     }
    // }
}
