pub mod chars;
pub mod palette;

pub struct Screen {
    pub pixels: [u8; ::fazic::SCREEN_PIXELS as usize],
    pub rgb_pixels: [u8; ::fazic::SCREEN_RGB_PIXELS as usize],
    pub current_color: u8,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; ::fazic::SCREEN_PIXELS as usize],
            rgb_pixels: [0; ::fazic::SCREEN_RGB_PIXELS as usize],
            current_color: 0,
        }
    }

    pub fn put_string(&mut self, string: String, mut x: u16, y: u16, color: u8) {
        println!("print({}, {}, {}, {})", string, x, y, color);

        for char in string.chars() {
            self.put_char(char, x, y, color, true);
            x = x + 8;
        }
    }

    pub fn put_char(&mut self, char: char, x: u16, y: u16, color: u8, reverse: bool) {
        let data = chars::get_char(char);

        for xx in 0..64 {
            if data & (0b1 << xx) != 0 {
                if !reverse {
                    self.put_pixel(x + (xx % 8), y + (xx / 8), color);
                }
            } else {
                if reverse {
                    self.put_pixel(x + (xx % 8), y + (xx / 8), color);
                }
            }
        }
    }

    pub fn set_current_color(&mut self, color: u8) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        let (r,g,b) = palette::rgb_for(self.current_color);

        for i in 0..::fazic::SCREEN_PIXELS {
            let i3 = i * 3;

            self.pixels[i] = self.current_color;
            self.rgb_pixels[i3]   = r;
            self.rgb_pixels[i3+1] = g;
            self.rgb_pixels[i3+2] = b;
        }
    }

    pub fn put_pixel(&mut self, x: u16, y: u16, color: u8) {
        // println!("putpixel({}, {}, {})", x, y, color);
        if x < ::fazic::SCREEN_WIDTH && y < ::fazic::SCREEN_HEIGHT {

            let i = x as usize + y as usize * ::fazic::SCREEN_WIDTH as usize;
            let i3 = i * 3;
            let (r,g,b) = palette::rgb_for(color);

            self.pixels[i] = color;
            self.rgb_pixels[i3]   = r;
            self.rgb_pixels[i3+1] = g;
            self.rgb_pixels[i3+2] = b;
        }
    }

    pub fn draw_text_buffer(&mut self, text : &::fazic::text_buffer::TextBuffer) {
        self.set_current_color(text.background_color);
        self.clear();

        for i in 0..::fazic::TEXT_BUFFER_CHARS {
            let is_cursor = text.cursor == i && text.show_cursor;

            let color = if is_cursor {
                text.current_color
            } else {
                text.colors[i as usize]
            };

            self.put_char(
                text.chars[i as usize],
                (i % ::fazic::TEXT_BUFFER_CHARS_PER_LINE * 8),
                (i / ::fazic::TEXT_BUFFER_CHARS_PER_LINE * 8),
                color,
                is_cursor,
                );
        }
    }
}
