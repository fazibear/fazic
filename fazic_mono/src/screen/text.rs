use std::path::Path;
use std::str::Chars;
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::render::{Renderer};

use screen::{colors};

pub struct Text<'t> {
    surface: Surface<'t>,
    string: &'t String,
    chars: String,
}

impl<'t> Text<'t> {
    pub fn new(string: &'t String) -> Text<'t> {
        let mut surface = match Surface::load_bmp(&Path::new("assets/chars.bmp")) {
            Ok(surface) => {
                surface
            },
            Err(err)    => panic!("failed to load surface: {}", err)
        };

        let _ = surface.set_color_key(true, colors::BLACK);

        let chars_string = "@abcdefghijklmnopqrstuvwxyz[£]^@ !\"#$%&'()*+,-./0123456789:;<=>?ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

        Text {
            surface: surface,
            string: string,
            chars: chars_string,
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for i in 0..80 {
            match self.string.chars().nth(i) {
                Some(c) => self.render_char(i, c, renderer),
                None => self.render_char(i, ' ', renderer),
            };
        };

    }

    fn render_char(&mut self, pos: usize, char: char, renderer: &mut Renderer) {
            self.surface.set_color_mod(colors::MAGENTA);

            let texture = match renderer.create_texture_from_surface(&self.surface) {
                Ok(texture) => texture,
                Err(err)    => panic!("failed to convert surface: {:?}", err)
            };

            let _ = renderer.copy(
                &texture,
                Some(self.get_char_rect(char)),
                Some(self.get_position_rect(pos)),
            );
    }

    fn get_char_rect(&self, char: char) -> Rect {
        let pos = self.chars.chars().position(|x| x == char).unwrap();

        let h = 16;
        let x = (pos % h * 8) as i32;
        let y = 128 + (pos / h * 8) as i32;

        Rect::new(x, y, 8, 8)
    }

    fn get_position_rect(&self, pos: usize) -> Rect {
        let h = 40;
        let x = 42 + (pos % h * 8) as i32;
        let y = 42 + (pos / h * 8) as i32;

        Rect::new(x, y, 8, 8)
    }
}
