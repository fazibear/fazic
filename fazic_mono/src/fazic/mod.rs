pub mod screen;
pub mod text_buffer;
pub mod ast;
pub mod runtime;
pub mod parser;

pub struct Fazic {
    screen: screen::Screen,
    text: text_buffer::TextBuffer,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text: text_buffer::TextBuffer::new(),
        }
    }

    pub fn redraw_text_buffer(&mut self) {
        runtime::text::redraw_buffer(self)
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
