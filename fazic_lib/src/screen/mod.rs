mod chars;

use config::*;
use std::mem;

pub struct Screen {
    pub pixels: [u8; SCREEN_PIXELS],
    pub current_color: u8,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            pixels: [0; SCREEN_PIXELS],
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

    pub fn put_char(&mut self, char: char, x: i32, y: i32, color: u8, reverse: bool) {
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
        for i in 0..SCREEN_PIXELS {
            self.pixels[i] = color;
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: u8) {
        if x >= 0 && y >= 0 && x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            let i = x as usize + y as usize * SCREEN_WIDTH as usize;
            self.pixels[i] = color;
        }
    }

    pub fn draw_text_buffer(&mut self, text: &::text_buffer::TextBuffer) {
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

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
        let mut x = x0;
        let mut y = y0;
        let mut y_longer = false;
        let mut short_len = y1 - y;
        let mut long_len = x1 - x;

        if short_len.abs() > long_len.abs() {
            mem::swap(&mut short_len, &mut long_len);
            y_longer = true;
        }

        let dec_inc = if long_len == 0 {
            0
        } else {
            (short_len << 16) / long_len
        };

        if y_longer {
            let mut j = 0x8000 + (x << 16);
            if long_len > 0 {
                long_len += y;
                while y <= long_len {
                    self.put_pixel(j >> 16, y, color);
                    j += dec_inc;
                    y += 1;
                }
                return;
            }
            long_len += y;
            while y >= long_len {
                self.put_pixel(j >> 16, y, color);
                j -= dec_inc;
                y -= 1;
            }
            return;
        }
        let mut j = 0x8000 + (y << 16);
        if long_len > 0 {
            long_len += x;
            while x <= long_len {
                self.put_pixel(x, j >> 16, color);
                j += dec_inc;
                x += 1;
            }
            return;
        }
        long_len += x;
        while x >= long_len {
            self.put_pixel(x, j >> 16, color);
            j -= dec_inc;
            x -= 1;
        }
    }

    pub fn circle(&mut self, x0: i32, y0: i32, radius: i32, color: u8) {
        let mut x = radius - 1;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - (radius << 1);

        while x >= y {
            self.put_pixel(x0 + x, y0 + y, color);
            self.put_pixel(x0 + y, y0 + x, color);
            self.put_pixel(x0 - y, y0 + x, color);
            self.put_pixel(x0 - x, y0 + y, color);
            self.put_pixel(x0 - x, y0 - y, color);
            self.put_pixel(x0 - y, y0 - x, color);
            self.put_pixel(x0 + y, y0 - x, color);
            self.put_pixel(x0 + x, y0 - y, color);

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }

            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - (radius << 1);
            }
        }
    }
}
