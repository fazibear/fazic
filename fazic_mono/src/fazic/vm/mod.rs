use std::time::Instant;
use ::fazic::enums::*;

mod commands;
mod expressions;
mod other;

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
                Instruction::Set(1, Value::Integer(0)),
                Instruction::Set(2, Value::Integer(15)),
                Instruction::Set(3, Value::Integer(1)),
                Instruction::Push(Stack::Next(1,2,3,5)),

                Instruction::Set(4, Value::Integer(0)),
                Instruction::Set(5, Value::Integer(240)),
                Instruction::Set(6, Value::Integer(1)),
                Instruction::Push(Stack::Next(4,5,6,9)),

                Instruction::Set(7, Value::Integer(0)),
                Instruction::Set(8, Value::Integer(320)),
                Instruction::Set(9, Value::Integer(1)),
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

    pub fn start(&mut self, tmp: bool, instructions: Vec<Instruction>) {
        self.instant = Instant::now();
        self.tmp_mode = tmp;
        if tmp {
            self.tmp_position = 0;
            self.tmp_instructions = instructions;
        } else {
            self.position = 0;
            self.instructions = instructions;
        }

         self.running = true;
     }

    pub fn set_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn cont(&mut self) {
        self.running = true;
    }

    pub fn jump(&mut self, value: usize) {
        if self.tmp_mode {
            self.tmp_position = value;
        } else {
            self.position = value;
        }
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

pub fn tmp_start(fazic: &mut ::fazic::Fazic, nodes: &[NodeElement]) {
    fazic.vm.start(true, ::fazic::compiler::compile(nodes, &mut fazic.variables));
}

pub fn start(fazic: &mut ::fazic::Fazic) {
    let mut nodes = vec![];
    fazic.program.nodes(&mut nodes);
    fazic.vm.start(false, ::fazic::compiler::compile(&nodes, &mut fazic.variables));
}

pub fn stop(fazic: &mut ::fazic::Fazic) {
    fazic.vm.running = false;
    fazic.mode = 0;
    fazic.text_buffer.prompt();
    println!("{:?}", fazic.vm.instant.elapsed());
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    match *fazic.vm.current() {
        Instruction::Run =>              start(fazic),
        Instruction::Stop =>             stop(fazic),
        Instruction::Jmp(pos) =>         fazic.vm.jump(pos),
        Instruction::JmpIf(pos, var) =>  other::jmpif(pos, var, fazic),

        Instruction::Pop =>              { fazic.stack.pop();                   fazic.vm.step() },
        Instruction::Push(_) =>          { other::push(fazic);                  fazic.vm.step() },

        Instruction::Mov(to, from) =>    { other::mov(to, from, fazic);         fazic.vm.step() },
        Instruction::Set(name, _) =>     { other::set_var(name, fazic);         fazic.vm.step() },

        Instruction::List =>             { commands::list(fazic);               fazic.vm.step() },
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
            let &Stack::Next(var, max, step, jmp) = fazic.stack.last().unwrap();

            expressions::add(var, step, var, fazic);
            expressions::lteq(var, max, 0, fazic);
            other::jmpif(jmp, 0, fazic);
        },
    }
}
