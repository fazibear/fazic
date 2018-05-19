mod chars;
mod palette;

use fazic::config::*;
use std::mem;

pub struct Screen {
    pub pixels: [u8; SCREEN_PIXELS as usize],
    pub rgb_pixels: [u8; SCREEN_RGB_PIXELS as usize],
    pub current_color: u8,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            pixels: [0; SCREEN_PIXELS as usize],
            rgb_pixels: [0; SCREEN_RGB_PIXELS as usize],
            current_color: 0,
        }
    }
}

impl Screen {
    pub fn new() -> Screen {
        Screen::default()
    }

    // pub fn put_string(&mut self, string: String, mut x: u16, y: u16, color: u8) {
    //
    //     for char in string.chars() {
    //         self.put_char(char, x, y, color, true);
    //         x += 8;
    //     }
    // }

    pub fn put_char(&mut self, char: char, x: u16, y: u16, color: u8, reverse: bool) {
        let data = chars::get_char(char);

        for xx in 0..64 {
            if data & (0b1 << xx) != 0 {
                if !reverse {
                    self.put_pixel(x + (xx % 8), y + (xx / 8), color);
                }
            } else if reverse {
                self.put_pixel(x + (xx % 8), y + (xx / 8), color);
            }
        }
    }

    // pub fn set_current_color(&mut self, color: u8) {
    //     self.current_color = color;
    // }

    pub fn clear(&mut self, color: u8) {
        let (r, g, b) = palette::rgb_for(color);

        for i in 0..SCREEN_PIXELS {
            let i3 = i * 3;

            self.pixels[i] = self.current_color;
            self.rgb_pixels[i3] = r;
            self.rgb_pixels[i3 + 1] = g;
            self.rgb_pixels[i3 + 2] = b;
        }
    }

    pub fn put_pixel(&mut self, x: u16, y: u16, color: u8) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            let i = x as usize + y as usize * SCREEN_WIDTH as usize;
            let i3 = i * 3;
            let (r, g, b) = palette::rgb_for(color);

            self.pixels[i] = color;
            self.rgb_pixels[i3] = r;
            self.rgb_pixels[i3 + 1] = g;
            self.rgb_pixels[i3 + 2] = b;
        }
    }

    pub fn draw_text_buffer(&mut self, text: &::fazic::text_buffer::TextBuffer) {
        self.clear(text.background_color);

        for i in 0..TEXT_BUFFER_CHARS {
            let is_cursor = text.cursor == i && text.show_cursor;

            let color = if is_cursor {
                text.current_color
            } else {
                text.colors[i as usize]
            };

            self.put_char(
                text.chars[i as usize],
                i % TEXT_BUFFER_CHARS_PER_LINE * 8,
                i / TEXT_BUFFER_CHARS_PER_LINE * 8,
                color,
                is_cursor,
            );
        }
    }

    pub fn line(&mut self, x: u16, y: u16, x2: u16, y2: u16, color: u8) {
        let mut x1 = x as i32;
        let mut y1 = y as i32;
        let mut y_longer = false;
        let mut short_len = y2 as i32 - y1;
        let mut long_len = x2 as i32 - x1;

        if short_len.abs() > long_len.abs() {
            mem::swap(&mut short_len, &mut long_len);
            y_longer = true;
        }

        let dec_inc = if long_len == 0 {
            0
        } else {
            ((short_len << 16) / long_len)
        };

        if y_longer {
            let mut j = 0x8000 + (x1 << 16);
            if long_len > 0 {
                long_len += y1;
                while y1 <= long_len {
                    self.put_pixel((j >> 16) as u16, y1 as u16, color);
                    j += dec_inc;
                    y1 += 1;
                }
                return;
            }
            long_len += y1;
            while y1 >= long_len {
                self.put_pixel((j >> 16) as u16, y1 as u16, color);
                j -= dec_inc;
                y1 -= 1;
            }
            return;
        }
        let mut j = 0x8000 + (y1 << 16);
        if long_len > 0 {
            long_len += x1;
            while x1 <= long_len {
                self.put_pixel(x1 as u16, (j >> 16) as u16, color);
                j += dec_inc;
                x1 += 1;
            }
            return;
        }
        long_len += x1;
        while x1 >= long_len {
            self.put_pixel(x1 as u16, (j >> 16) as u16, color);
            j -= dec_inc;
            x1 -= 1;
        }
    }
}
