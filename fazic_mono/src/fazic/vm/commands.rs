use fazic::vm::Value;
use std::mem;

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
    let mut x = match *fazic.variables.get(x) {
        Value::Number(x) => x as i32,
        _ => 0,
    };
    let mut y = match *fazic.variables.get(y) {
        Value::Number(y) => y as i32,
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

    let mut y_longer = false;
    let mut short_len = y2 - y;
    let mut long_len = x2 - x;
    if short_len.abs() > long_len.abs() {
        mem::swap(&mut short_len, &mut long_len);
        y_longer = true;
    }

    let dec_inc = if long_len == 0 {
        0
    } else {
        (short_len << 16) / long_len
    };


    if y_longer {
        let mut j = 0x8000 + (x << 16);
        if long_len > 0 {
            long_len += y;
            while y<=long_len {
              fazic.screen.put_pixel((j >> 16) as u16, y as u16, color);
              j += dec_inc;
              y += 1;
            }
            return;
        }
        long_len += y;
        while y >=long_len {
          fazic.screen.put_pixel((j >> 16) as u16, y as u16, color);
          j -= dec_inc;
          y -= 1;
        }
        return;
    }
    let mut j = 0x8000 + (y << 16);
    if long_len > 0 {
        long_len += x;
        while x <= long_len {
            fazic.screen.put_pixel(x as u16, (j >> 16) as u16, color);
            j += dec_inc;
            x += 1;
        }
        return;
    }
    long_len += x;
    while x >= long_len {
        fazic.screen.put_pixel(x as u16, (j >> 16) as u16, color);
        j -= dec_inc;
        x -= 1;
    }
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
