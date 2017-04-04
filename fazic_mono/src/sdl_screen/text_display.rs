extern crate sdl2;
use std::path::Path;
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::render::{Renderer, Texture};

use sdl_screen::{colors};
use sdl_screen::colors::{Color};
use runtime::text_buffer::{TextBuffer};

pub struct Text<'t> {
    pub buffer: &'t mut TextBuffer,
    pub renderer: Box<Renderer<'t>>,
    texture: Texture,
}

impl<'t> Text<'t> {
    pub fn new(renderer: Box<Renderer<'t>>, buffer: &'t mut TextBuffer) -> Text<'t> {
        let mut surface = match Surface::load_bmp(&Path::new("assets/chars.bmp")) {
            Ok(surface) => {
                surface
            },
            Err(err)    => panic!("failed to load surface: {}", err)
        };

        let _ = surface.set_color_key(true, colors::BLACK);

        let texture = match renderer.create_texture_from_surface(surface) {
            Ok(texture) => texture,
            Err(err)    => panic!("failed to convert surface: {:?}", err)
        };

        Text {
            buffer: buffer,
            renderer: renderer,
            texture: texture,
        }
    }

    pub fn render(&mut self, blink: bool) {
        for i in 0..1000 {
            self.render_char(i, blink)
        };
    }

    fn render_char(&mut self, pos: usize, blink: bool) {
        let is_cursor = blink && (pos == self.buffer.cursor);

        let color = if is_cursor {
            self.get_current_color().value()
        } else {
            self.get_color(pos).value()
        };

        match color {
            sdl2::pixels::Color::RGB(r, g, b) => self.texture.set_color_mod(r,g,b),
            sdl2::pixels::Color::RGBA(r, g, b, _) => self.texture.set_color_mod(r,g,b),
        }

        let a = self.get_char_rect(self.buffer.chars[pos], is_cursor);
        let b = self.get_position_rect(pos);

        let _ = self.renderer.copy(
            &self.texture, Some(a), Some(b)
        );
    }

    fn get_char_rect(&self, char: char, rev: bool) -> Rect {
        let pos = match self.buffer.get_char_index(char) {
            Some(pos) => pos,
                    _ => 32,
        };

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
        self.match_color(self.buffer.colors[pos])
    }

    fn get_current_color(&self) -> Color {
        self.match_color(self.buffer.current_color)
    }

    fn match_color(&self, color: usize) -> Color {
        match color {
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
             _ => unreachable!(),
        }
    }
}
