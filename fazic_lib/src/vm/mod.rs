use enums::*;
use std::time::Instant;

mod commands;
mod expressions;
mod functions;
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
        self.tmp_mode = false;
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

pub fn start(fazic: &mut ::Fazic) {
    let mut nodes = vec![];
    fazic.program.nodes(&mut nodes);
    fazic.vm.start(
        false,
        ::compiler::compile(&nodes, &mut fazic.variables, &mut fazic.lines, false),
    );
}

pub fn stop(fazic: &mut ::Fazic) {
    if fazic.vm.running {
        fazic.vm.running = false;
        fazic.mode = 0;

        let line = fazic.lines.what_line(fazic.vm.position);
        let message = format!("? BREAK IN {}", line);

        fazic.text_buffer.insert_line(&message);
        fazic.text_buffer.prompt();
        debug!("{:?}", fazic.vm.instant.elapsed());
    }
}

pub fn end(fazic: &mut ::Fazic) {
    fazic.vm.running = false;
    fazic.mode = 0;
    fazic.text_buffer.prompt();
    debug!("{:?}", fazic.vm.instant.elapsed());
}

pub fn onerror(fazic: &mut ::Fazic) {
    if let Instruction::Error(ref msg) = fazic.vm.current().clone() {
        error(fazic, msg);
    }
}

pub fn error(fazic: &mut ::Fazic, msg: &str) {
    let line = fazic.lines.what_line(fazic.vm.position);
    let message = format!("? {} IN LINE {}", msg, line);

    fazic.text_buffer.insert_line(&message);
    fazic.vm.running = false;
    fazic.mode = 0;
    fazic.text_buffer.prompt();
    debug!("{:?}", fazic.vm.instant.elapsed());
}

pub fn step(fazic: &mut ::Fazic) {
    match *fazic.vm.current() {
        Instruction::Error(_) => onerror(fazic),
        Instruction::Run => start(fazic),
        Instruction::End => end(fazic),
        Instruction::Stop => stop(fazic),
        Instruction::Cont => fazic.vm.cont(),
        Instruction::Clr => {
            fazic.variables = ::variables::Variables::new();
            fazic.vm.step()
        }
        Instruction::New => {
            fazic.program = ::program::Program::new();
            end(fazic)
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
        Instruction::Dir => {
            commands::dir(fazic);
            fazic.vm.step()
        }
        Instruction::Load(var) => {
            commands::load(var, fazic);
            end(fazic)
        }
        Instruction::Save(var) => {
            commands::save(var, fazic);
            end(fazic)
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
        Instruction::Clear(var) => {
            commands::clear(var, fazic);
            fazic.vm.step()
        }
        Instruction::Srand(var) => {
            commands::srand(var, fazic);
            fazic.vm.step()
        }
        Instruction::Dot(x, y) => {
            commands::dot(x, y, fazic);
            fazic.vm.step()
        }
        Instruction::Line(x, y, x2, y2) => {
            commands::line(x, y, x2, y2, fazic);
            fazic.vm.step()
        }
        Instruction::Circle(x, y, r) => {
            commands::circle(x, y, r, fazic);
            fazic.vm.step()
        }
        Instruction::Mode(mode) => {
            commands::mode(mode, fazic);
            fazic.vm.step()
        }
        Instruction::And(a, b, dst) => {
            expressions::and(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Or(a, b, dst) => {
            expressions::or(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Add(a, b, dst) => {
            expressions::add(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Sub(a, b, dst) => {
            expressions::sub(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Mul(a, b, dst) => {
            expressions::mul(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Div(a, b, dst) => {
            expressions::div(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Mod(a, b, dst) => {
            expressions::mod_(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Pow(a, b, dst) => {
            expressions::pow(a, b, dst, fazic);
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
        Instruction::GtEq(a, b, dst) => {
            expressions::gteq(a, b, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Neg(a, dst) => {
            expressions::neg(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Not(a, dst) => {
            expressions::not(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Rnd(a, dst) => {
            functions::rnd(a, dst, fazic);
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
        Instruction::Atn(a, dst) => {
            functions::atn(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Exp(a, dst) => {
            functions::exp(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Log(a, dst) => {
            functions::log(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Sqr(a, dst) => {
            functions::sqr(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Sgn(a, dst) => {
            functions::sgn(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Len(a, dst) => {
            functions::len(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Chr(a, dst) => {
            functions::chr(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Asc(a, dst) => {
            functions::asc(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Val(a, dst) => {
            functions::val(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Int(a, dst) => {
            functions::int(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Str(a, dst) => {
            functions::str(a, dst, fazic);
            fazic.vm.step()
        }
        Instruction::Time(dst) => {
            functions::time(dst, fazic);
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
