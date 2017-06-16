pub mod screen;
pub mod text_buffer;
pub mod config;
pub mod ast;
pub mod stack;
pub mod node_builder;
pub mod program;
pub mod execute;
pub mod vm;

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

pub fn parse(fazic: &mut ::fazic::Fazic, input: String) {
    match parser::parse_all(&input) {
        Ok(ast::Entry(None, nodes)) => {
            println!("{:?}", nodes);
            execute::exec_each_node(nodes, fazic);
            if !fazic.program.running {
                fazic.text_buffer.prompt();
            }
        },
        Ok(ast::Entry(Some(line), nodes)) => {
             fazic.program.add_line(line as u16, nodes, input.clone());
        },
        Err(e) => {
            println!("Parse error!: {:?}", e);
            fazic.text_buffer.insert_line(&format!("{: >1$}", "^", e.column));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    };
}

pub struct Fazic {
    screen: screen::Screen,
    pub text_buffer: text_buffer::TextBuffer,
    program: program::Program,
    pub redraw: bool,
    mode: u8,
    vm: vm::VM,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text_buffer: text_buffer::TextBuffer::new(),
            program: program::Program::new(),
            redraw: true,
            mode: 0,
            vm: vm::VM::new(),
        }
    }

    fn text_mode(&mut self) -> bool {
        self.mode == 0
    }

    // fn flip_mode(&mut self) -> bool {
    //     self.mode == 1
    // }

    fn instant_mode(&mut self) -> bool {
        self.mode == 2
    }

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
            if input.len() == 0 {
                self.text_buffer.insert_line("");
                return;
            }
            self.text_buffer.enter();
            parse(self, input)
        }
    }

    pub fn stop_key(&mut self) {
        execute::commands::stop(self);
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

    pub fn redraw(&mut self) {
        if self.instant_mode() {
            self.redraw = true;
        }
    }

    pub fn need_to_redraw(&mut self) -> bool {
        if self.redraw {
            self.redraw = false;
            true
        } else {
            false
        }
    }

    pub fn tick_1(&mut self) {
        if self.program.running {
            //for _ in 0..500 {
                execute::exec_node(self.program.current_node(), self);
            //}
            if !self.program.running {
                self.text_buffer.prompt();
            }
        }
        if self.text_mode() && self.text_buffer.changed {
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            self.redraw = true;
        }
    }

    pub fn tick_2(&mut self) {
        if self.vm.running {
            //for _ in 0..500 {
                vm::step(self);
            //}
            if !self.vm.running {
                self.text_buffer.prompt();
            }
        }
        if self.text_mode() && self.text_buffer.changed {
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            self.redraw = true;
        }
    }
}
