use ::fazic::vm::Value;

pub fn print(var: usize, fazic: &mut ::fazic::Fazic) {
    println!("print: {:?}", fazic.variables[var]);

    let string = match fazic.variables[var] {
        Value::String(ref s) => s.to_string(), //format!("{}", s),
        Value::Integer(i) => format!("{}", i),
        Value::Float(f) => format!("{}", f),
        Value::Bool(b) => format!("{}", b),
        Value::Null => "null".to_string(),
    };

    fazic.text_buffer.insert_line(&string);
}

pub fn color(var: usize, fazic: &mut ::fazic::Fazic) {
    let color = match fazic.variables[var] {
        Value::Integer(i) => i as u8,
        Value::Float(f) => f as u8,
        _ => 0,
    };
    fazic.screen.current_color = color;
    fazic.text_buffer.current_color = color;
}

pub fn dot(x: usize, y: usize, fazic: &mut ::fazic::Fazic) {
    let x = match fazic.variables[x] {
        Value::Integer(x) => x as u16,
        Value::Float(x) => x as u16,
        _ => 0
    };

    let y = match fazic.variables[y] {
        Value::Integer(y) => y as u16,
        Value::Float(y) => y as u16,
        _ => 0
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
