pub mod screen;
pub mod text_buffer;
pub mod runtime;

pub const SCREEN_WIDTH: u16 = 320;
pub const SCREEN_HEIGHT: u16 = 240;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
pub const SCREEN_RGB_PIXELS: usize = SCREEN_PIXELS * 3;

pub const TEXT_BUFFER_CHARS_PER_LINE: u16 = SCREEN_WIDTH / 8;
pub const TEXT_BUFFER_LINES: u16 = SCREEN_HEIGHT / 8;
pub const TEXT_BUFFER_CHARS: u16 = TEXT_BUFFER_CHARS_PER_LINE * TEXT_BUFFER_LINES;

pub const TEXT_BUFFER_MAX_LINES: u16 = TEXT_BUFFER_LINES * 1000;
pub const TEXT_BUFFER_MAX_LINE_CHARS: u16 = TEXT_BUFFER_CHARS_PER_LINE * 5;

pub const BASIC_MAX_LINES: u16 = 10000;

pub struct Fazic {
    screen: screen::Screen,
    pub text_buffer: text_buffer::TextBuffer,
    program: runtime::program::Program,
    redraw: bool,
    text_mode: bool,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text_buffer: text_buffer::TextBuffer::new(),
            program: runtime::program::Program::new(),
            redraw: true,
            text_mode: true,
        }
    }

    pub fn set_current_text_color(&mut self, color: u8) {
        if self.text_mode {
            self.text_buffer.set_current_color(color);
        }
    }

    pub fn up_key(&mut self) {
        if self.text_mode {
            self.text_buffer.up()
        }
    }

    pub fn down_key(&mut self) {
        if self.text_mode {
            self.text_buffer.down()
        }
    }

    pub fn left_key(&mut self) {
        if self.text_mode {
            self.text_buffer.left()
        }
    }

    pub fn right_key(&mut self) {
        if self.text_mode {
            self.text_buffer.right()
        }
    }

    pub fn backspace_key(&mut self) {
        if self.text_mode {
            self.text_buffer.backspace()
        }
    }

    pub fn enter_key(&mut self) {
        if self.text_mode {
            runtime::exec(self);
        }
    }

    pub fn stop_key(&mut self) {
        runtime::execute::commands::stop(self);
    }

    pub fn insert_string(&mut self, string: String) {
        if self.text_mode {
            self.text_buffer.insert_string(string)
        }
    }

    pub fn blink_cursor(&mut self) {
        if self.text_mode {
            self.text_buffer.blink_cursor()
        }
    }

    pub fn get_rgb_pixels(&mut self) -> &mut [u8] {
        &mut self.screen.rgb_pixels
    }

    pub fn redraw(&mut self) -> bool {
        self.redraw
    }

    pub fn redrawed(&mut self) {
        self.redraw = false
    }

    pub fn tick(&mut self) {
        runtime::step(self);
        if self.text_mode && self.text_buffer.changed {
            self.screen.draw_text_buffer(&self.text_buffer);
            self.text_buffer.refreshed();
            self.redraw = true;
        }
    }
}
