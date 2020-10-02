#[macro_use]
extern crate log;
extern crate peg;
extern crate rand;

mod compiler;
mod lines;
mod parser;
mod program;
mod screen;
mod text_buffer;
mod variables;
mod vm;

pub mod config;
pub mod enums;
pub mod nodes;

use rand::SeedableRng;
use rand::XorShiftRng;
use std::time::Instant;

#[cfg(test)]
mod tests;

pub fn parse(fazic: &mut ::Fazic, input: &str) {
    match parser::parser::parse_all(input) {
        Ok(nodes::Entry(None, nodes)) => {
            fazic.vm.start(
                true,
                ::compiler::compile(&nodes, &mut fazic.variables, &mut fazic.lines, true),
            );
        }
        Ok(nodes::Entry(Some(line), nodes)) => {
            if nodes.is_empty() {
                fazic.program.remove_line(line as u16);
            } else {
                fazic.program.add_line(line as u16, nodes, input);
            }
        }
        Err(e) => {
            debug!("Parse error!: {:?}", e);
            fazic
                .text_buffer
                .insert_line(&format!("{: >1$}", "^", e.location.column));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    };
}
pub enum DrawCallback {
    PutPixel(i32, i32, u8, u8, u8),
    Clear(u8, u8, u8),
    Redraw(),
}

pub enum FileSystemCallback {
    Load(String),
    Save(String),
}

pub struct Fazic {
    text_buffer: text_buffer::TextBuffer,
    program: program::Program,
    mode: u8,
    vm: vm::VM,
    variables: variables::Variables,
    stack: Vec<enums::Stack>,
    lines: lines::Lines,
    instant: Instant,
    rng: XorShiftRng,
    current_color: u8,
    draw_callback: Option<Box<dyn FnMut(DrawCallback)>>,
    file_system_callback: Option<Box<dyn FnMut(FileSystemCallback)>>,
    redraw: bool,
}

impl Default for Fazic {
    fn default() -> Fazic {
        Fazic {
            text_buffer: text_buffer::TextBuffer::new(),
            program: program::Program::new(),
            mode: 0,
            vm: vm::VM::new(),
            variables: variables::Variables::new(),
            stack: Vec::with_capacity(100),
            lines: lines::Lines::new(),
            instant: Instant::now(),
            rng: SeedableRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            current_color: 0,
            draw_callback: None,
            file_system_callback: None,
            redraw: true,
        }
    }
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic::default()
    }

    // Callbacks

    pub fn set_draw_callback(&mut self, c: Box<dyn FnMut(DrawCallback)>) {
        self.draw_callback = Some(c);
    }

    pub fn draw_callback(&mut self, action: DrawCallback) {
        match self.draw_callback {
            Some(ref mut draw) => draw(action),
            None => (),
        };
    }

    pub fn set_file_system_callback(&mut self, c: Box<dyn FnMut(FileSystemCallback)>) {
        self.file_system_callback = Some(c);
    }

    fn text_mode(&mut self) -> bool {
        self.mode == 0
    }

    // fn flip_mode(&mut self) -> bool {
    //     self.mode == 1
    // }

    //fn instant_mode(&mut self) -> bool {
    //    self.mode == 2
    //}

    pub fn set_current_text_color(&mut self, color: u8) {
        if self.text_mode() {
            self.text_buffer.set_current_color(color);
        }
    }

    pub fn up_key(&mut self) {
        if self.text_mode() {
            self.text_buffer.up()
        }
    }

    pub fn down_key(&mut self) {
        if self.text_mode() {
            self.text_buffer.down()
        }
    }

    pub fn left_key(&mut self) {
        if self.text_mode() {
            self.text_buffer.left()
        }
    }

    pub fn right_key(&mut self) {
        if self.text_mode() {
            self.text_buffer.right()
        }
    }

    pub fn backspace_key(&mut self) {
        if self.text_mode() {
            self.text_buffer.backspace()
        }
    }

    pub fn enter_key(&mut self) {
        if self.text_mode() {
            let input = self.text_buffer.get_current_line_string();
            if input.is_empty() {
                self.text_buffer.insert_line("");
                return;
            }
            self.text_buffer.enter();
            parse(self, &input)
        }
    }

    pub fn stop_key(&mut self) {
        vm::stop(self);
    }

    pub fn insert_string(&mut self, string: String) {
        if self.text_mode() {
            self.text_buffer.insert_string(string)
        }
    }

    pub fn blink_cursor(&mut self) {
        if self.text_mode() {
            self.text_buffer.blink_cursor()
        }
    }

    //pub fn redraw(&mut self) {
    //    if self.instant_mode() {
    //        self.screen.redraw = true;
    //    }
    //}

    pub fn tick(&mut self) {
        if self.vm.running {
            vm::step(self);
        }
        if self.text_mode() && self.text_buffer.changed {
            ::screen::draw_text_buffer(self);
            self.text_buffer.refreshed();
            self.draw_callback(DrawCallback::Redraw())
        };
    }
}
