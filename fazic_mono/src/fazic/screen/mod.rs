pub mod chars;
pub mod palette;

pub const WIDTH: u16 = 320;
pub const HEIGHT: u16 = 240;
pub const PIXELS: usize = WIDTH as usize * HEIGHT as usize;
pub const RGB_PIXELS: usize = PIXELS * 3;

pub struct Screen {
    pub pixels: [u8; PIXELS as usize],
    pub rgb_pixels: Box<[u8; RGB_PIXELS as usize]>,
    pub current_color: u8,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; PIXELS as usize],
            rgb_pixels: Box::new([0; RGB_PIXELS as usize]),
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

        for i in 0..PIXELS {
            let i3 = i * 3;

            self.pixels[i] = self.current_color;
            self.rgb_pixels[i3]   = r;
            self.rgb_pixels[i3+1] = g;
            self.rgb_pixels[i3+2] = b;
        }
    }

    pub fn put_pixel(&mut self, x: u16, y: u16, color: u8) {
        // println!("putpixel({}, {}, {})", x, y, color);
        if x < WIDTH && y < HEIGHT {

            let i = x as usize + y as usize * WIDTH as usize;
            let i3 = i * 3;
            let (r,g,b) = palette::rgb_for(color);

            self.pixels[i] = color;
            self.rgb_pixels[i3]   = r;
            self.rgb_pixels[i3+1] = g;
            self.rgb_pixels[i3+2] = b;
        }
    }
}
