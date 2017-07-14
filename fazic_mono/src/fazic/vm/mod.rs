use std::time::Instant;
use ::fazic::enums::*;

pub mod commands;
pub mod expressions;
pub mod other;

pub struct VM {
    pub instructions: Vec<Instruction>,
    pub position: usize,

    pub running: bool,
    pub instant: Instant,

    pub tmp_instructions: Vec<Instruction>,
    pub tmp_position: usize,
    pub tmp_mode: bool,
}

impl Default for VM {
    fn default() -> VM {
        VM {
            instructions: vec![
                Instruction::Mode(1),
                Instruction::SetVar(1, Value::Integer(0)),
                Instruction::SetVar(2, Value::Integer(1)),
                Instruction::SetVar(3, Value::Integer(15)),
                Instruction::Push(Stack::Next(1,2,3,5)),

                Instruction::SetVar(4, Value::Integer(0)),
                Instruction::SetVar(5, Value::Integer(1)),
                Instruction::SetVar(6, Value::Integer(240)),
                Instruction::Push(Stack::Next(4,5,6,9)),

                Instruction::SetVar(7, Value::Integer(0)),
                Instruction::SetVar(8, Value::Integer(1)),
                Instruction::SetVar(9, Value::Integer(320)),
                Instruction::Push(Stack::Next(7,8,9,13)),

                Instruction::Dot(7,4),

                Instruction::Next,
                Instruction::Pop,

                Instruction::Next,
                Instruction::Pop,

                Instruction::Flip,
                Instruction::Color(1),

                Instruction::Next,
                Instruction::Pop,

                Instruction::Stop
            ],
            position: 0,

            running: false,
            instant: Instant::now(),

            tmp_instructions: vec![],
            tmp_position: 0,
            tmp_mode: false,
        }
    }
}

impl VM {
    pub fn new() -> VM {
        VM::default()
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
        if self.tmp_mode {
            self.tmp_position += 1;
        } else {
            self.position += 1;
        }
    }

    pub fn current(&mut self) -> &Instruction {
        if self.tmp_mode {
            &self.tmp_instructions[self.tmp_position]
        } else {
            &self.instructions[self.position]
        }
    }
}

// fn error(msg: String, fazic: &mut ::fazic::Fazic) {
//     let msg = format!("? {}", msg);
//     fazic.text_buffer.insert_line(&msg);
//     fazic.text_buffer.prompt();
//     fazic.vm.stop();
// }
//


pub fn step(fazic: &mut ::fazic::Fazic) {
    match *fazic.vm.current() {
        Instruction::Stop =>             fazic.vm.stop(),

        Instruction::Jmp(pos) =>         { other::jmp(pos, fazic); },
        Instruction::JmpIf(pos, var) =>  { other::jmpif(pos, var, fazic); },

        Instruction::Pop =>              { fazic.stack.pop();                fazic.vm.step() },
        Instruction::Push(_) =>          { other::push(fazic);                  fazic.vm.step() },
        Instruction::SetVar(name, _) =>  { other::set_var(name, fazic);         fazic.vm.step() },

        Instruction::Flip =>             { commands::flip(fazic);               fazic.vm.step() },
        Instruction::Print(var) =>       { commands::print(var, fazic);         fazic.vm.step() },
        Instruction::Color(var) =>       { commands::color(var, fazic);         fazic.vm.step() },
        Instruction::Dot(x, y) =>        { commands::dot(x, y, fazic);          fazic.vm.step() },
        Instruction::Mode(mode) =>       { commands::mode(mode, fazic);         fazic.vm.step() },

        Instruction::Add(a, b, dst) =>   { expressions::add(a, b, dst, fazic);  fazic.vm.step() },
        Instruction::Gt(a, b, dst) =>    { expressions::gt(a, b, dst, fazic);   fazic.vm.step() },
        Instruction::Lt(a, b, dst) =>    { expressions::lt(a, b, dst, fazic);   fazic.vm.step() },
        Instruction::LtEq(a, b, dst) =>  { expressions::lteq(a, b, dst, fazic); fazic.vm.step() },

        Instruction::Next =>             {
            let &Stack::Next(var, step, max, jmp) = fazic.stack.last().unwrap();

            expressions::add(var, step, var, fazic);
            expressions::lteq(var, max, 0, fazic);
            other::jmpif(jmp, 0, fazic);
        },
    }
}
