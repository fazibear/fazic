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

pub struct Fazic {
    screen: screen::Screen,
    text: text_buffer::TextBuffer,
    program: runtime::program::Program,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text: text_buffer::TextBuffer::new(),
            program: runtime::program::Program::new(),
        }
    }

    pub fn redraw_text_buffer(&mut self) {
        self.screen.draw_text_buffer(&self.text)
    }

    pub fn set_current_text_color(&mut self, color: u8) {
        self.text.set_current_color(color)
    }

    pub fn up_key(&mut self) {
        self.text.up()
    }

    pub fn down_key(&mut self) {
        self.text.down()
    }

    pub fn left_key(&mut self) {
        self.text.left()
    }

    pub fn right_key(&mut self) {
        self.text.right()
    }

    pub fn backspace_key(&mut self) {
        self.text.backspace()
    }

    pub fn enter_key(&mut self) {
        runtime::exec(
            &self.program,
            &self.text.get_current_line_string()
        );
        self.text.enter()
    }

    pub fn insert_string(&mut self, string: String) {
        self.text.insert_string(string)
    }

    pub fn blink_cursor(&mut self) {
        self.text.blink_cursor()
    }

    pub fn get_rgb_pixels(&mut self) -> &mut [u8] {
        &mut *self.screen.rgb_pixels
    }

    pub fn tick(&mut self) {
        if self.text.changed {
            self.redraw_text_buffer();
            self.text.refreshed();
        }
    }
}
