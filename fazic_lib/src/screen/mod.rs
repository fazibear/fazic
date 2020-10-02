mod chars;
mod palette;

use config::*;
use std::mem;
use DrawCallback;

pub fn put_char(fazic: &mut ::Fazic, char: char, x: i32, y: i32, color: u8, reverse: bool) {
    let data = chars::get_char(char);

    for xx in 0..64 {
        if data & (0b1 << xx) != 0 {
            if !reverse {
                put_pixel(fazic, x + (xx % 8), y + (xx / 8), color);
            }
        } else if reverse {
            put_pixel(fazic, x + (xx % 8), y + (xx / 8), color);
        }
    }
}

pub fn clear(fazic: &mut ::Fazic, color: u8) {
    let (r, g, b) = palette::rgb_for(color);
    fazic.draw_callback(DrawCallback::Clear(r, g, b));
}

pub fn put_pixel(fazic: &mut ::Fazic, x: i32, y: i32, color: u8) {
    let (r, g, b) = palette::rgb_for(color);
    fazic.draw_callback(DrawCallback::PutPixel(x, y, r, g, b));
}

pub fn draw_text_buffer(fazic: &mut ::Fazic) {
    clear(fazic, fazic.text_buffer.background_color);

    for i in 0..TEXT_BUFFER_CHARS {
        let is_cursor = fazic.text_buffer.cursor == i && fazic.text_buffer.show_cursor;

        let color = if is_cursor {
            fazic.text_buffer.current_color
        } else {
            fazic.text_buffer.colors[i as usize]
        };

        put_char(
            fazic,
            fazic.text_buffer.chars[i as usize],
            i % TEXT_BUFFER_CHARS_PER_LINE * 8,
            i / TEXT_BUFFER_CHARS_PER_LINE * 8,
            color,
            is_cursor,
        );
    }
}

pub fn line(fazic: &mut ::Fazic, x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
    let mut x = x0 as i32;
    let mut y = y0 as i32;
    let mut y_longer = false;
    let mut short_len = y1 as i32 - y;
    let mut long_len = x1 as i32 - x;

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
                put_pixel(fazic, j >> 16, y, color);
                j += dec_inc;
                y += 1;
            }
            return;
        }
        long_len += y;
        while y >= long_len {
            put_pixel(fazic, j >> 16, y, color);
            j -= dec_inc;
            y -= 1;
        }
        return;
    }
    let mut j = 0x8000 + (y << 16);
    if long_len > 0 {
        long_len += x;
        while x <= long_len {
            put_pixel(fazic, x, j >> 16, color);
            j += dec_inc;
            x += 1;
        }
        return;
    }
    long_len += x;
    while x >= long_len {
        put_pixel(fazic, x, j >> 16, color);
        j -= dec_inc;
        x -= 1;
    }
}

pub fn circle(fazic: &mut ::Fazic, x0: i32, y0: i32, radius: i32, color: u8) {
    let mut x = radius - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1);

    while x >= y {
        put_pixel(fazic, x0 + x, y0 + y, color);
        put_pixel(fazic, x0 + y, y0 + x, color);
        put_pixel(fazic, x0 - y, y0 + x, color);
        put_pixel(fazic, x0 - x, y0 + y, color);
        put_pixel(fazic, x0 - x, y0 - y, color);
        put_pixel(fazic, x0 - y, y0 - x, color);
        put_pixel(fazic, x0 + y, y0 - x, color);
        put_pixel(fazic, x0 + x, y0 - y, color);

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
