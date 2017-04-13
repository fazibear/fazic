pub mod screen;
pub mod text_buffer;
pub mod ast;
pub mod runtime;
pub mod parser;

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
        redraw_text_buffer(self)
    }

    pub fn tick(&mut self) {
        if self.text.changed {
            self.redraw_text_buffer();
            self.text.refreshed();
        }
    }
}

fn redraw_text_buffer(runtime : &mut Fazic) {
        runtime.screen.set_current_color(runtime.text.background_color);
        runtime.screen.clear();

        for i in 0..text_buffer::CHARS {
            let is_cursor = runtime.text.cursor == i && runtime.text.show_cursor;

            let color = if is_cursor {
                runtime.text.current_color
            } else {
                runtime.text.colors[i as usize]
            };

            runtime.screen.put_char(
                runtime.text.chars[i as usize],
                (i % 40 * 8),
                (i / 40 * 8),
                color,
                is_cursor,
            );
        }
}
