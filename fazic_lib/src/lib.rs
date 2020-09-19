#[macro_use]
extern crate log;
extern crate rand;
extern crate simple_logger;

mod compiler;
mod lines;
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

mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

pub fn parse(fazic: &mut ::Fazic, input: &str) {
    match parser::parse_all(input) {
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
                .insert_line(&format!("{: >1$}", "^", e.column));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    };
}
pub enum DrawAction {
    PutPixel(i32, i32, u8, u8, u8),
    Clear(u8, u8, u8),
    Redraw(),
}

pub struct Fazic {
    screen: screen::Screen,
    text_buffer: text_buffer::TextBuffer,
    program: program::Program,
    mode: u8,
    vm: vm::VM,
    variables: variables::Variables,
    stack: Vec<enums::Stack>,
    lines: lines::Lines,
    instant: Instant,
    rng: XorShiftRng,
}

impl Default for Fazic {
    fn default() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text_buffer: text_buffer::TextBuffer::new(),
            program: program::Program::new(),
            mode: 0,
            vm: vm::VM::new(),
            variables: variables::Variables::new(),
            stack: Vec::with_capacity(100),
            lines: lines::Lines::new(),
            instant: Instant::now(),
            rng: SeedableRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        }
    }
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic::default()
    }

    // Callbacks

    pub fn set_draw_callback(&mut self, c: Box<dyn FnMut(DrawAction)>) {
        self.screen.set_draw_callback(c);
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

    pub fn get_rgb_pixels(&mut self) -> &mut [u8] {
        &mut self.screen.rgb_pixels
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
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            match self.screen.callback_draw {
                Some(ref mut draw) => draw(DrawAction::Redraw()),
                None => (),
            };
        };
    }
}
