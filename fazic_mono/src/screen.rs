
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

    pub fn char(&self, char: char, x: usize, y: usize) {

    }

    pub fn putpixel(&mut self, x: usize, y: usize, color: u8) {
        println!("putpixel({}, {}, {})", x, y, color);

        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        self.pixels[x + y * SCREEN_WIDTH] = color;
    }
}
