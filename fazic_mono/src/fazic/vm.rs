use std::time::Instant;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub enum Instruction {
    Print(usize),
    Dot(usize, usize),
    Jmp(usize),
    JmpIf(usize, usize),
    Mode(u8),
    SetVar(usize, Value),
    Add(usize, usize, usize),
    Gt(usize, usize, usize),
    Lt(usize, usize, usize),
    LtEq(usize, usize, usize),
    Flip,
    Color(usize),
    Stop,
}


pub struct VM {
    pub instructions: Vec<Instruction>,
    pub position: usize,
    pub running: bool,
    pub variables: Vec<Value>,
    pub instant: Instant,
}

impl VM {
    pub fn new() -> VM {
        VM {
            instructions: vec![
                Instruction::Mode(1),
                Instruction::SetVar(1, Value::Integer(0)),
                Instruction::SetVar(2, Value::Integer(1)),
                Instruction::SetVar(3, Value::Integer(15)),

                Instruction::SetVar(4, Value::Integer(0)),
                Instruction::SetVar(5, Value::Integer(1)),
                Instruction::SetVar(6, Value::Integer(240)),

                Instruction::SetVar(7, Value::Integer(0)),
                Instruction::SetVar(8, Value::Integer(1)),
                Instruction::SetVar(9, Value::Integer(320)),

                Instruction::Dot(7,4),

                Instruction::Add(7, 8, 7),
                Instruction::LtEq(7, 9, 0),
                Instruction::JmpIf(10, 0),

                Instruction::Add(4, 5, 4),
                Instruction::LtEq(4, 6, 0),
                Instruction::JmpIf(7, 0),

                Instruction::Flip,
                Instruction::Color(1),

                Instruction::Add(1, 2, 1),
                Instruction::LtEq(1, 3, 0),
                Instruction::JmpIf(4, 0),

                Instruction::Stop
            ],
            position: 0,
            running: false,
            variables: vec![Value::Null; 100],
            instant: Instant::now(),

        }
    }

    pub fn set_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn start(&mut self) {
        self.position = 0;
        self.running = true;
        self.instant = Instant::now();
    }

    pub fn cont(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
        println!("{:?}", self.instant.elapsed());
    }

}

// fn error(msg: String, fazic: &mut ::fazic::Fazic) {
//     let msg = format!("? {}", msg);
//     fazic.text_buffer.insert_line(&msg);
//     fazic.text_buffer.prompt();
//     fazic.vm.stop();
// }

fn get_var(variables: &Vec<Value>, variable: usize) -> Value {
    // println!("get: {}={:?}", variable, variables[variable]);
    return variables[variable].clone()
}

fn set_var(variables: & mut Vec<Value>, variable: usize, value: &Value) {
    variables[variable] = value.clone();
    //println!("set: {}={:?}", variable, variables[variable]);
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    match fazic.vm.instructions[fazic.vm.position] {
        Instruction::Print(var) => {
            println!("print: {:?}", get_var(&fazic.vm.variables, var));

            let string = match get_var(&fazic.vm.variables, var) {
                Value::String(ref s) => format!("{}", s),
                Value::Integer(i) => format!("{}", i),
                Value::Float(f) => format!("{}", f),
                Value::Bool(b) => format!("{}", b),
                Value::Null => "null".to_string(),
            };

            fazic.text_buffer.insert_line(&string);
            fazic.vm.position += 1;
        },
        Instruction::Color(var) => {
            let color = match get_var(&fazic.vm.variables, var) {
                Value::Integer(i) => i as u8,
                Value::Float(f) => f as u8,
                _ => 0,
            };

            fazic.screen.current_color = color;
            fazic.text_buffer.current_color = color;
            fazic.vm.position += 1;
        },
        Instruction::Dot(x, y) => {
            let (x,y) = match (get_var(&fazic.vm.variables, x), get_var(&fazic.vm.variables, y)) {
                (Value::Integer(x), Value::Integer(y)) => ((x as u16), (y as u16)),
                (Value::Float(x), Value::Float(y)) => ((x as u16), (y as u16)),
                (Value::Float(x), Value::Integer(y)) => ((x as u16), (y as u16)),
                (Value::Integer(x), Value::Float(y)) => ((x as u16), (y as u16)),
                _ => (0,0)
            };

            let color = fazic.screen.current_color;

            fazic.screen.put_pixel(x, y, color);
            fazic.vm.position += 1;
        },
        Instruction::Jmp(position) => {
            fazic.vm.position = position;
        },
        Instruction::JmpIf(position, var) => {
            let cond = match get_var(&fazic.vm.variables, var) {
                Value::String(_) => false,
                Value::Integer(i) if i == 0 => false,
                Value::Integer(_) => true,
                Value::Float(f) if f == 0.0 => false,
                Value::Float(_) => true,
                Value::Bool(b) => b,
                Value::Null => false,
            };

            if cond {
                fazic.vm.position = position
            } else {
                fazic.vm.position += 1
            };
        },
        Instruction::Flip => {
            fazic.redraw = true;
            fazic.vm.position += 1;
        },
        Instruction::Stop => {
            fazic.vm.running = false;
            println!("{:?}", fazic.vm.instant.elapsed());
        },
        Instruction::Mode(mode) => {
            fazic.mode = mode;
            fazic.vm.position += 1;
        }
        Instruction::SetVar(name, ref value) => {
            set_var(&mut fazic.vm.variables, name, value);
            fazic.vm.position += 1;
        },
        Instruction::Add(a, b, dst) => {
            let ret = match (get_var(&fazic.vm.variables, a), get_var(&fazic.vm.variables, b)) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
                (Value::Float(l), Value::Integer(r)) => Value::Float(l + r as f64),
                (Value::Integer(l), Value::Float(r)) => Value::Float(l as f64 + r),
                (Value::String(l), Value::String(r)) => {
                    let mut str = l.clone();
                    str.push_str(&r);
                    Value::String(str)
                },
                (_, _) => {
                    // error("TYPE MISMATCH".to_string(), fazic);
                    Value::Null
                },
            };
            set_var(&mut fazic.vm.variables, dst, &ret);
            fazic.vm.position += 1;
        },
        Instruction::Gt(a, b, dst) => {
            let ret = match (get_var(&fazic.vm.variables, a), get_var(&fazic.vm.variables, b)) {
                (Value::Integer(l), Value::Integer(r)) => Value::Bool(l > r),
                (Value::Float(l), Value::Float(r)) => Value::Bool(l > r),
                (Value::Float(l), Value::Integer(r)) => Value::Bool(l > (r as f64)),
                (Value::Integer(l), Value::Float(r)) => Value::Bool((l as f64) > r),
                (_, _) => {
                    // error("TYPE MISMATCH".to_string(), fazic);
                    Value::Null
                },
            };
            set_var(&mut fazic.vm.variables, dst, &ret);
            fazic.vm.position += 1;
        },
        Instruction::Lt(a, b, dst) => {
            let ret = match (get_var(&fazic.vm.variables, a), get_var(&fazic.vm.variables, b)) {
                (Value::Integer(l), Value::Integer(r)) => Value::Bool(l < r),
                (Value::Float(l), Value::Float(r)) => Value::Bool(l < r),
                (Value::Float(l), Value::Integer(r)) => Value::Bool(l < (r as f64)),
                (Value::Integer(l), Value::Float(r)) => Value::Bool((l as f64) < r),
                (_, _) => {
                    // error("TYPE MISMATCH".to_string(), fazic);
                    Value::Null
                },
            };
            set_var(&mut fazic.vm.variables, dst, &ret);
            fazic.vm.position += 1;
        },
        Instruction::LtEq(a, b, dst) => {
            let ret = match (get_var(&fazic.vm.variables, a), get_var(&fazic.vm.variables, b)) {
                (Value::Integer(l), Value::Integer(r)) => Value::Bool(l <= r),
                (Value::Float(l), Value::Float(r)) => Value::Bool(l <= r),
                (Value::Float(l), Value::Integer(r)) => Value::Bool(l <= (r as f64)),
                (Value::Integer(l), Value::Float(r)) => Value::Bool((l as f64) <= r),
                (_, _) => {
                    // error("TYPE MISMATCH".to_string(), fazic);
                    Value::Null
                },
            };
            set_var(&mut fazic.vm.variables, dst, &ret);
            fazic.vm.position += 1;
        },
    }
}
