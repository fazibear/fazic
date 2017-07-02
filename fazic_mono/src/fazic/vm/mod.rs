use std::time::Instant;

pub mod commands;
pub mod expressions;

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

    pub fn step(&mut self) {
        self.position += 1;
    }

    pub fn current(&mut self) -> &Instruction {
        &self.instructions[self.position]
    }
}

// fn error(msg: String, fazic: &mut ::fazic::Fazic) {
//     let msg = format!("? {}", msg);
//     fazic.text_buffer.insert_line(&msg);
//     fazic.text_buffer.prompt();
//     fazic.vm.stop();
// }


pub fn set_var(name: usize, fazic: &mut ::fazic::Fazic) {
    fazic.vm.variables[name] = match fazic.vm.current() {
        &Instruction::SetVar(_, ref val) => val.clone(),
        _ => unreachable!(),
    };
}

pub fn jmp(pos: usize, fazic: &mut ::fazic::Fazic) {
    fazic.vm.position = pos;
}

pub fn jmpif(pos: usize, var: usize, fazic: &mut ::fazic::Fazic) {
    let cond = match &fazic.vm.variables[var] {
        &Value::String(_) => false,
        &Value::Integer(i) if i == 0 => false,
        &Value::Integer(_) => true,
        &Value::Float(f) if f == 0.0 => false,
        &Value::Float(_) => true,
        &Value::Bool(b) => b,
        &Value::Null => false,
    };

    if cond {
        fazic.vm.position = pos
    } else {
        fazic.vm.position += 1
    };
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    match fazic.vm.current() {
        &Instruction::Stop =>             fazic.vm.stop(),
        &Instruction::Jmp(pos) =>         { jmp(pos, fazic); },
        &Instruction::JmpIf(pos, var) =>  { jmpif(pos, var, fazic); },
        &Instruction::SetVar(name, _) =>  { set_var(name, fazic); fazic.vm.step() },

        &Instruction::Flip =>             { commands::flip(fazic);               fazic.vm.step() },
        &Instruction::Print(var) =>       { commands::print(var, fazic);         fazic.vm.step() },
        &Instruction::Color(var) =>       { commands::color(var, fazic);         fazic.vm.step() },
        &Instruction::Dot(x, y) =>        { commands::dot(x, y, fazic);          fazic.vm.step() },
        &Instruction::Mode(mode) =>       { commands::mode(mode, fazic);         fazic.vm.step() },

        &Instruction::Add(a, b, dst) =>   { expressions::add(a, b, dst, fazic);  fazic.vm.step() },
        &Instruction::Gt(a, b, dst) =>    { expressions::gt(a, b, dst, fazic);   fazic.vm.step() },
        &Instruction::Lt(a, b, dst) =>    { expressions::lt(a, b, dst, fazic);   fazic.vm.step() },
        &Instruction::LtEq(a, b, dst) =>  { expressions::lteq(a, b, dst, fazic); fazic.vm.step() },
    }
}
