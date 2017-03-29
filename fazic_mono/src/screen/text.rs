use std::path::Path;
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::render::{Renderer};

use screen::{colors};

pub struct Text<'t> {
    surface: Surface<'t>,
    string: &'t String,
}

impl<'t> Text<'t> {
    pub fn new(string: &'t String) -> Text<'t> {
        let mut surface = match Surface::load_bmp(&Path::new("assets/chars.bmp")) {
            Ok(surface) => surface,
            Err(err)    => panic!("failed to load surface: {}", err)
        };
        Text {
            surface: surface,
            string: string,
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for i in 0..80 {
            self.surface.set_color_key(true, colors::BLACK);
            self.surface.set_color_mod(colors::MAGENTA);

            let char: char = match self.string.chars().nth(i) {
                Some(c) => c,
                None => ' ',
            };

            let texture = match renderer.create_texture_from_surface(&self.surface) {
                Ok(texture) => texture,
                Err(err)    => panic!("failed to convert surface: {:?}", err)
            };

            println!("{}", char);
            let char = Rect::new(0, 0, 8, 8);
            let pos = Rect::new(42, 42, 8, 8);

            let _ = renderer.copy(&texture, Some(char), Some(pos));
        };

    }
}
