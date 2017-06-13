use std::collections::HashMap;
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
    Print(String),
    Dot(String, String),
    Jmp(usize),
    JmpIf(usize, String),
    Mode(u8),
    SetVar(String, Value),
    Add(String, String, String),
    Gt(String, String, String),
    Lt(String, String, String),
    LtEq(String, String, String),
    Flip,
    Color(String),
    Stop,
}


pub struct VM {
    pub instructions: Vec<Instruction>,
    pub position: usize,
    pub running: bool,
    pub variables: HashMap<String, Value>,
    pub instant: Instant,
}

impl VM {
    pub fn new() -> VM {
        VM {
            instructions: vec![
                Instruction::Mode(1),
                Instruction::SetVar("C".to_string(), Value::Integer(0)),
                Instruction::SetVar("_FOR_STEP_C".to_string(), Value::Integer(1)),
                Instruction::SetVar("_FOR_TO_C".to_string(), Value::Integer(15)),

                Instruction::SetVar("Y".to_string(), Value::Integer(0)),
                Instruction::SetVar("_FOR_STEP_Y".to_string(), Value::Integer(1)),
                Instruction::SetVar("_FOR_TO_Y".to_string(), Value::Integer(240)),

                Instruction::SetVar("X".to_string(), Value::Integer(0)),
                Instruction::SetVar("_FOR_STEP_X".to_string(), Value::Integer(1)),
                Instruction::SetVar("_FOR_TO_X".to_string(), Value::Integer(320)),

                Instruction::Dot("X".to_string(), "Y".to_string()),

                Instruction::Add("X".to_string(), "_FOR_STEP_X".to_string(), "X".to_string()),
                Instruction::LtEq("X".to_string(), "_FOR_TO_X".to_string(), "TMP".to_string()),
                Instruction::JmpIf(10, "TMP".to_string()),

                Instruction::Add("Y".to_string(), "_FOR_STEP_Y".to_string(), "Y".to_string()),
                Instruction::LtEq("Y".to_string(), "_FOR_TO_Y".to_string(), "TMP".to_string()),
                Instruction::JmpIf(7, "TMP".to_string()),

                Instruction::Flip,
                Instruction::Color("C".to_string()),

                Instruction::Add("C".to_string(), "_FOR_STEP_C".to_string(), "C".to_string()),
                Instruction::LtEq("C".to_string(), "_FOR_TO_C".to_string(), "TMP".to_string()),
                Instruction::JmpIf(4, "TMP".to_string()),

                Instruction::Stop
            ],
            position: 0,
            running: false,
            variables: HashMap::new(),
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

fn get_var(variables: &HashMap<String, Value>, variable: &str) -> Value {
    match variables.get(variable) {
        Some(value) => value.clone(),
        None => Value::Integer(0),
    }
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    match fazic.vm.instructions[fazic.vm.position] {
        Instruction::Print(ref var) => {
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
        Instruction::Color(ref var) => {
            let color = match get_var(&fazic.vm.variables, var) {
                Value::Integer(i) => i as u8,
                Value::Float(f) => f as u8,
                _ => 0,
            };

            fazic.screen.current_color = color;
            fazic.text_buffer.current_color = color;
            fazic.vm.position += 1;
        },
        Instruction::Dot(ref x, ref y) => {
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
        Instruction::JmpIf(position, ref var) => {
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
        Instruction::SetVar(ref name, ref value) => {
            fazic.vm.variables.insert((*name).clone(), (*value).clone());
            fazic.vm.position += 1;
        },
        Instruction::Add(ref a, ref b, ref dst) => {
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
            fazic.vm.variables.insert((*dst).clone(), ret.clone());
            fazic.vm.position += 1;
        },
        Instruction::Gt(ref a, ref b, ref dst) => {
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
            fazic.vm.variables.insert((*dst).clone(), ret.clone());
            fazic.vm.position += 1;
        },
        Instruction::Lt(ref a, ref b, ref dst) => {
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
            fazic.vm.variables.insert((*dst).clone(), ret.clone());
            fazic.vm.position += 1;
        },
        Instruction::LtEq(ref a, ref b, ref dst) => {
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
            fazic.vm.variables.insert((*dst).clone(), ret.clone());
            fazic.vm.position += 1;
        },
    }
}
