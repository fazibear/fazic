pub mod screen;
pub mod text_buffer;

pub struct Fazic {
    pub screen: screen::Screen,
    pub text: text_buffer::TextBuffer,
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new(),
            text: text_buffer::TextBuffer::new(),
        }
    }

    pub fn redraw_text_buffer(&mut self) {
        self.screen.set_current_color(self.text.background_color);
        self.screen.clear();

        for i in 0..text_buffer::CHARS {
            let is_cursor = self.text.cursor == i && self.text.show_cursor;

            let color = if is_cursor {
                self.text.current_color
            } else {
                self.text.colors[i as usize]
            };

            self.screen.put_char(
                self.text.chars[i as usize],
                (i % 40 * 8),
                (i / 40 * 8),
                color,
                is_cursor,
            );
        }
    }

    pub fn tick(&mut self) {
        if self.text.changed {
            self.redraw_text_buffer();
            self.text.refreshed();
        }
    }
}
