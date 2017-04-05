pub mod chars;
pub mod palette;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const PIXELS: usize = WIDTH * HEIGHT;

pub struct Screen {
    pub pixels: [u8; PIXELS],
    pub rgb_pixels: Box<[u8; PIXELS * 3]>,
    pub current_color: u8,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; PIXELS],
            rgb_pixels: Box::new([0; PIXELS * 3]),
            current_color: 0,
        }
    }

    pub fn print(&mut self, string: String, mut x: usize, y: usize, color: u8) {
        println!("print({}, {}, {}, {})", string, x, y, color);

        for char in string.chars() {
            let data = chars::get_char(char);

            for xx in 0..64 {
                if data & (0b1 << xx) != 0 {
                    self.putpixel(x + (xx % 8), y + (xx / 8), color);
                }
            }
            x = x + 8;
        }
    }

    pub fn set_current_color(&mut self, color: u8) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        let (r,g,b) = palette::rgb_for(self.current_color);

        for i in 0..PIXELS {
            self.pixels[i] = self.current_color;
            self.rgb_pixels[i*3]   = r;
            self.rgb_pixels[i*3+1] = g;
            self.rgb_pixels[i*3+2] = b;
        }
    }

    pub fn putpixel(&mut self, x: usize, y: usize, color: u8) {
        // println!("putpixel({}, {}, {})", x, y, color);
        if x < WIDTH && y < HEIGHT {

            let i = x + y * WIDTH;
            let (r,g,b) = palette::rgb_for(color);

            self.pixels[i] = color;
            self.rgb_pixels[i*3]   = r;
            self.rgb_pixels[i*3+1] = g;
            self.rgb_pixels[i*3+2] = b;
        }
    }
}
