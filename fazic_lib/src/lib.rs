#[macro_use]
extern crate log;
extern crate peg;
extern crate rand;

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

mod compiler;
mod enums;
mod lines;
mod nodes;
mod parser;
mod program;
mod screen;
mod text_buffer;
mod variables;
mod vm;

pub mod config;
pub mod file_system;
pub mod rtc;
pub mod colors;

use rand::SeedableRng;
use rand::XorShiftRng;

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Fazic {
    current_color: u8,
    rtc: Box<dyn rtc::Rtc>,
    lines: lines::Lines,
    mode: u8,
    program: program::Program,
    file_system: Box<dyn file_system::FileSystem>,
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
            file_system: Box::new(file_system::MemoryFileSystem::new()),
            rtc: Box::new(rtc::DummyRtc::new()),
            current_color: 0,
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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Fazic {
    pub fn new() -> Fazic {
        Fazic::default()
    }

    fn is_text_mode(&mut self) -> bool {
        self.mode == 0
    }

    pub fn set_file_system(&mut self, fs: Box<dyn file_system::FileSystem>) {
        self.file_system = fs;
    }
    
    pub fn set_rtc(&mut self, rtc: Box<dyn rtc::Rtc>) {
        self.rtc = rtc;
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
            self.parse(&input)
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

    pub fn get_pixels(&mut self) -> &mut [u8] {
        &mut self.screen.pixels
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
    
    pub fn parse(&mut self, input: &str) {
        match parser::parser::parse_all(input) {
            Ok(nodes::Entry(None, nodes)) => {
                self.vm.start(
                    true,
                    ::compiler::compile(&nodes, &mut self.variables, &mut self.lines, true),
                );
            }
            Ok(nodes::Entry(Some(line), nodes)) => {
                if nodes.is_empty() {
                    self.program.remove_line(line as u16);
                } else {
                    self.program.add_line(line as u16, nodes, input);
                }
            }
            Err(e) => {
                debug!("Parse error!: {:?}", e);
                self
                    .text_buffer
                    .insert_line(&format!("{: >1$}", "^", e.location.column));
                self.text_buffer.insert_line("?SYNTAX ERROR");
                self.text_buffer.prompt();
            }
        };
    }
}
