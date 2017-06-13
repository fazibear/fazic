use std::collections::HashMap;

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
    Jmp(usize),
    Mode(u8),
    SetVar(String, Value),
    Stop,
}


pub struct VM {
    pub instructions: Vec<Instruction>,
    pub position: usize,
    pub running: bool,
    pub variables: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            instructions: vec![
                Instruction::SetVar("X".to_string(), Value::String("test".to_string())),
                Instruction::Print("Dupa".to_string()),
                Instruction::Jmp(1)
            ],
            position: 0,
            running: false,
            variables: HashMap::new(),
        }
    }

    pub fn set_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn start(&mut self) {
        self.position = 0;
        self.running = true;
    }

    pub fn cont(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

}

pub fn step(fazic: &mut ::fazic::Fazic) {
        match fazic.vm.instructions[fazic.vm.position] {
            Instruction::Print(ref string) => {
                fazic.text_buffer.insert_line(string);
                fazic.vm.position += 1;
            },
            Instruction::Jmp(position) => {
                fazic.vm.position = position;
            },
            Instruction::Stop => {
                fazic.vm.running = false;
            },
            Instruction::Mode(mode) => {
                fazic.mode = mode;
                fazic.vm.position += 1;
            }
            Instruction::SetVar(ref name, ref value) => {
                fazic.vm.variables.insert((*name).clone(), (*value).clone());
                fazic.vm.position += 1;
            },
        }
    }
