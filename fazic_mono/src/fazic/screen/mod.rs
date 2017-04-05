pub mod chars;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const PIXELS: usize = WIDTH * HEIGHT;

pub struct Screen {
    pub pixels: [u8; PIXELS]
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; PIXELS]
        }
    }

    pub fn print(&mut self, string: String, mut x: usize, y: usize, color: u8) {
        println!("print({}, {}, {}, {})", string, x, y, color);

        for char in string.chars() {
            let data = chars::get_c64_char(char);

            for xx in 0..64 {
                if data & (0b1 << xx) != 0 {
                    self.putpixel(x + (xx % 8), y + (xx / 8), color);
                }
            }
            x = x + 8;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..PIXELS {
            self.pixels[i] = 0;
        }
    }

    pub fn putpixel(&mut self, x: usize, y: usize, color: u8) {
        // println!("putpixel({}, {}, {})", x, y, color);

        if x < WIDTH && y < HEIGHT {
            self.pixels[x + y * WIDTH] = color;
        }
    }
}
