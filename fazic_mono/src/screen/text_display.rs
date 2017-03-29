use std::path::Path;
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::render::{Renderer};

use screen::{colors};
use screen::colors::{Color};
use runtime::text_buffer::{TextBuffer};

pub struct Text<'t> {
    surface: Surface<'t>,
    chars: String,
    buffer: &'t TextBuffer,
}

impl<'t> Text<'t> {
    pub fn new(buffer: &'t TextBuffer) -> Text<'t> {
        let mut surface = match Surface::load_bmp(&Path::new("assets/chars.bmp")) {
            Ok(surface) => {
                surface
            },
            Err(err)    => panic!("failed to load surface: {}", err)
        };

        let _ = surface.set_color_key(true, colors::BLACK);

        let chars_string = "@abcdefghijklmnopqrstuvwxyz[£]^@ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

        Text {
            surface: surface,
            chars: chars_string,
            buffer: buffer,
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for i in 0..1040 {
            match self.buffer.string.chars().nth(i) {
                Some(c) => self.render_char(i, c, renderer),
                None => self.render_char(i, ' ', renderer),
            };
        };
    }

    fn render_char(&mut self, pos: usize, char: char, renderer: &mut Renderer) {
        let color = self.get_color(pos).value();
        self.surface.set_color_mod(color);

        let texture = match renderer.create_texture_from_surface(&self.surface) {
            Ok(texture) => texture,
            Err(err)    => panic!("failed to convert surface: {:?}", err)
        };

        let _ = renderer.copy(
            &texture,
            Some(self.get_char_rect(char, pos == self.buffer.cursor)),
            Some(self.get_position_rect(pos)),
        );
    }

    fn get_char_rect(&self, char: char, rev: bool) -> Rect {
        let pos = self.chars.chars().position(|x| x == char).unwrap();

        let h = 16;
        let x = (pos % h * 8) as i32;
        let mut y = 128 + (pos / h * 8) as i32;

        if rev { y += 64 }

        Rect::new(x, y, 8, 8)
    }

    fn get_position_rect(&self, pos: usize) -> Rect {
        let h = 40;
        let x = 42 + (pos % h * 8) as i32;
        let y = 42 + (pos / h * 8) as i32;

        Rect::new(x, y, 8, 8)
    }

    fn get_color(&self, pos: usize) -> Color {
        match self.buffer.colors[pos] {
             0 => Color::Black,
             1 => Color::White,
             2 => Color::Red,
             3 => Color::Cyan,
             4 => Color::Magenta,
             5 => Color::Green,
             6 => Color::Blue,
             7 => Color::Yellow,
             8 => Color::Orange,
             9 => Color::Brown,
            10 => Color::Pink,
            11 => Color::DarkGrey,
            12 => Color::Grey,
            13 => Color::LightGreen,
            14 => Color::LightBlue,
            15 => Color::LightGrey,
             _ => Color::White,
        }
    }
}
