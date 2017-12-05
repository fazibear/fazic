use std::time::Instant;
use fazic::enums::*;

mod commands;
mod expressions;
mod other;
mod functions;

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
            instructions: vec![],
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

pub fn start(fazic: &mut ::fazic::Fazic) {
    let mut nodes = vec![];
    fazic.program.nodes(&mut nodes);
    fazic.vm.start(
        false,
        ::fazic::compiler::compile(&nodes, &mut fazic.variables, &mut fazic.lines),
    );
}

pub fn stop(fazic: &mut ::fazic::Fazic) {
    fazic.vm.running = false;
    fazic.mode = 0;
    fazic.text_buffer.prompt();
    println!("{:?}", fazic.vm.instant.elapsed());
}

pub fn onerror(fazic: &mut ::fazic::Fazic) {
    if let Instruction::Error(ref msg) = fazic.vm.current().clone() {
        error(fazic, msg);
    }
}

pub fn error(fazic: &mut ::fazic::Fazic, msg: &str) {
    let line = fazic.lines.what_line(fazic.vm.position);
    let message = format!("? {} IN LINE {}", msg, line);

    fazic.text_buffer.insert_line(&message);
    fazic.vm.running = false;
    fazic.mode = 0;
    fazic.text_buffer.prompt();
    println!("{:?}", fazic.vm.instant.elapsed());
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    match *fazic.vm.current() {
        Instruction::Error(_) => onerror(fazic),
        Instruction::Run => start(fazic),
        Instruction::End => stop(fazic),
        Instruction::Clr => {
            fazic.variables = ::fazic::variables::Variables::new();
            fazic.vm.step()
        }
        Instruction::New => {
            fazic.program = ::fazic::program::Program::new();
            stop(fazic)
        }
        Instruction::JmpLine(_) => unreachable!(),
        Instruction::JmpIfNotNextLine(_, _) => unreachable!(),
        Instruction::Jmp(pos) => fazic.vm.jump(pos),
        Instruction::JmpIfNot(pos, var) => other::jmpifnot(pos, var, fazic),
        Instruction::Pop => {
            fazic.stack.pop();
            fazic.vm.step()
        }
        Instruction::Push(_) => {
            other::push(fazic);
            fazic.vm.step()
        }

        Instruction::Mov(to, from) => {
            other::mov(to, from, fazic);
            fazic.vm.step()
        }
        Instruction::Set(name, _) => {
            other::set_var(name, fazic);
            fazic.vm.step()
        }

        Instruction::List => {
            commands::list(fazic);
            fazic.vm.step()
        }
        Instruction::Load(var) => {
            commands::load(var, fazic);
            stop(fazic)
        }
        Instruction::Save(var) => {
            commands::save(var, fazic);
            stop(fazic)
        }
        Instruction::Flip => {
            commands::flip(fazic);
            fazic.vm.step()
        }
        Instruction::Print(var) => {
            commands::print(var, fazic);
            fazic.vm.step()
        }
        Instruction::Color(var) => {
            commands::color(var, fazic);
            fazic.vm.step()
        }
        Instruction::Dot(x, y) => {
            commands::dot(x, y, fazic);
            fazic.vm.step()
        }
        Instruction::Mode(mode) => {
            commands::mode(mode, fazic);
            fazic.vm.step()
        }

        Instruction::Add(a, b, dst) => {
            expressions::add(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Eq(a, b, dst) => {
            expressions::eq(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Gt(a, b, dst) => {
            expressions::gt(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Lt(a, b, dst) => {
            expressions::lt(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::LtEq(a, b, dst) => {
            expressions::lteq(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Neg(a, dst) => {
            expressions::neg(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Rng(a, dst) => {
            functions::rng(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Abs(a, dst) => {
            functions::abs(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Sin(a, dst) => {
            functions::sin(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Cos(a, dst) => {
            functions::cos(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Tan(a, dst) => {
            functions::tan(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Next => match fazic.stack.last() {
            Some(&Stack::Next(var, max, step, jmp)) => {
                expressions::add(var, step, var, fazic);
                expressions::lteq(var, max, 0, fazic);
                other::jmpif(0, jmp, fazic);
            }
            _ => error(fazic, "NEXT WITHOUT FOR"),
        },
        Instruction::Return => match fazic.stack.pop() {
            Some(Stack::Return(pos)) => fazic.vm.jump(pos),
            _ => error(fazic, "RETEURN WITHOUT GOSUB"),
        },
    }
}
