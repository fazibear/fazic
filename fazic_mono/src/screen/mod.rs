extern crate sdl2;

use sdl2::event::{Event};
//use sdl2::surface::{Surface};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};

pub mod colors;
pub mod text_display;

const SCREEN_SCALE: f32 = 2.0;

const WINDOW_WIDTH: u32 = (404 as f32 * SCREEN_SCALE) as u32;
const WINDOW_HEIGHT: u32 = (284 as f32 * SCREEN_SCALE) as u32;

const SCREEN_Y: i32 = 42;
const SCREEN_X: i32 = 42;
const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx
        .window("fazic", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut renderer = match window
        .renderer()
        .build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

    let string = "                                                     **** FAZIC ****                                                    READY.".to_string();
    let mut text = text_display::Text::new(&string);

    let mut events = ctx.event_pump().unwrap();

    let mut running = true;
    while running {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let _ = renderer.set_scale(SCREEN_SCALE, SCREEN_SCALE);
        let _ = renderer.set_draw_color(colors::LIGHT_BLUE);
        let _ = renderer.clear();

        let _ = renderer.set_draw_color(colors::BLUE);

        let screen_rect = Rect::new(SCREEN_X, SCREEN_Y, SCREEN_WIDTH, SCREEN_HEIGHT);
        let _ = renderer.fill_rect(screen_rect);

        text.render(&mut renderer);
        let _ = renderer.present();
    }
}
