pub mod screen;
pub mod text_buffer;
pub mod runtime;
pub mod config;

pub struct Fazic {
    screen: screen::Screen,
    pub text_buffer: text_buffer::TextBuffer,
    program: runtime::program::Program,
    redraw: bool,
    mode: u8,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text_buffer: text_buffer::TextBuffer::new(),
            program: runtime::program::Program::new(),
            redraw: true,
            mode: 0,
        }
    }

    fn text_mode(&mut self) -> bool {
        self.mode == 0
    }

    fn flip_mode(&mut self) -> bool {
        self.mode == 1
    }

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
            runtime::exec(self);
        }
    }

    pub fn stop_key(&mut self) {
        runtime::execute::commands::stop(self);
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

    pub fn tick(&mut self) {
        runtime::step(self);
        if self.text_mode() && self.text_buffer.changed {
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            self.redraw = true;
        }
    }
}
