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

pub enum FileSystemCallback {
    Load(String),
    Save(String, String),
    Dir(),
}

pub struct Fazic {
    current_color: u8,
    file_system_callback: Option<Box<dyn FnMut(FileSystemCallback) -> Result<String, String>>>,
    instant: Instant,
    lines: lines::Lines,
    mode: u8,
    program: program::Program,
    redraw: bool,
    rng: XorShiftRng,
    screen: screen::Screen,
    stack: Vec<enums::Stack>,
    text_buffer: text_buffer::TextBuffer,
    variables: variables::Variables,
    vm: vm::VM,
}

impl Default for Fazic {
    fn default() -> Fazic {
        Fazic {
            current_color: 0,
            file_system_callback: None,
            instant: Instant::now(),
            lines: lines::Lines::new(),
            mode: 0,
            program: program::Program::new(),
            redraw: true,
            rng: SeedableRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            screen: screen::Screen::new(),
            stack: Vec::with_capacity(100),
            text_buffer: text_buffer::TextBuffer::new(),
            variables: variables::Variables::new(),
            vm: vm::VM::new(),
        }
    }
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic::default()
    }

    // Callbacks

    pub fn set_file_system_callback(
        &mut self,
        c: Box<dyn FnMut(FileSystemCallback) -> Result<String, String>>,
    ) {
        self.file_system_callback = Some(c);
    }

    pub fn file_system_callback(&mut self, action: FileSystemCallback) -> Result<String, String> {
        match self.file_system_callback {
            Some(ref mut fs) => fs(action),
            None => Err("CALLBACK ERROR".to_string()),
        }
    }

    fn is_text_mode(&mut self) -> bool {
        self.mode == 0
    }

    // fn is_flip_mode(&mut self) -> bool {
    //     self.mode == 1
    // }

    // fn is_instant_mode(&mut self) -> bool {
    //    self.mode == 2
    // }

    pub fn set_current_text_color(&mut self, color: u8) {
        if self.is_text_mode() {
            self.text_buffer.set_current_color(color);
        }
    }

    pub fn up_key(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.up()
        }
    }

    pub fn down_key(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.down()
        }
    }

    pub fn left_key(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.left()
        }
    }

    pub fn right_key(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.right()
        }
    }

    pub fn backspace_key(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.backspace()
        }
    }

    pub fn enter_key(&mut self) {
        if self.is_text_mode() {
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
        if self.is_text_mode() {
            self.text_buffer.insert_string(string)
        }
    }

    pub fn blink_cursor(&mut self) {
        if self.is_text_mode() {
            self.text_buffer.blink_cursor()
        }
    }
    
    pub fn get_rgb_pixels(&mut self) -> &mut [u8] {
        &mut self.screen.rgb_pixels
    }
    
    pub fn need_to_redraw(&mut self) -> bool {
        if self.redraw {
            self.redraw = false;
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.vm.running {
            vm::step(self);
        }
        if self.is_text_mode() && self.text_buffer.changed {
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            self.redraw = true;
        }
        self.redraw

    }
}
