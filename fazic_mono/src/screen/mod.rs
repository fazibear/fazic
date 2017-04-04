pub mod chars;


pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Screen {
    pub pixels: [u8; SCREEN_PIXELS]
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; SCREEN_PIXELS]
        }
    }

    pub fn print(&mut self, string: String, x: usize, y: usize, color: u8) {
        println!("print({}, {}, {}, {})", string, x, y, color);

        let mut x = x;

        for char in string.chars() {
            let data = chars::get_c64_char(char);

            for xx in 0..63 {
                if data & (0b1 << xx) != 0 {
                    self.putpixel(x + (xx % 8), y + (xx / 8), color);
                }
            }
            x = x + 8;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..SCREEN_PIXELS {
            self.pixels[i] = 0;
        }
    }

    pub fn putpixel(&mut self, x: usize, y: usize, color: u8) {
        println!("putpixel({}, {}, {})", x, y, color);

        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        self.pixels[x + y * SCREEN_WIDTH] = color;
    }
}
