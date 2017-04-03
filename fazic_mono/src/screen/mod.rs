extern crate sdl2;

pub mod colors;
pub mod text_display;

use std::process;
use sdl2::event::{Event};
use sdl2::keyboard::*;

use sdl2::rect::{Rect};

use runtime::text_buffer::{TextBuffer};
use self::text_display::{Text};

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

    let window = match video_ctx
        .window("fazic", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let renderer = match window
        .renderer()
        .build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

    let mut text_buffer = TextBuffer::new();
    let mut text = Text::new(Box::new(renderer), &mut text_buffer);
    let mut events = ctx.event_pump().unwrap();
    let mut timer = ctx.timer().unwrap();

    let mut ms_passed = 0;
    let mut blink = false;

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(key), keymod: LGUIMOD, ..} |
                Event::KeyDown { keycode: Some(key), keymod: RGUIMOD, ..} => {
                    match key {
                        Keycode::Num1 => text.buffer.set_current_color(0),
                        Keycode::Num2 => text.buffer.set_current_color(1),
                        Keycode::Num3 => text.buffer.set_current_color(2),
                        Keycode::Num4 => text.buffer.set_current_color(3),
                        Keycode::Num5 => text.buffer.set_current_color(4),
                        Keycode::Num6 => text.buffer.set_current_color(5),
                        Keycode::Num7 => text.buffer.set_current_color(6),
                        Keycode::Num8 => text.buffer.set_current_color(7),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    match key {
                        Keycode::Left => text.buffer.left(),
                        Keycode::Right => text.buffer.right(),
                        Keycode::Up => text.buffer.up(),
                        Keycode::Down => text.buffer.down(),
                        _ => (),
                    }
                },
                Event::TextInput { text: string, ..} => {
                    match string.chars().nth(0) {
                        Some(char) => text.buffer.set_char(char),
                        _ => (),
                    }
                }
                _ => ()
            }
        }

        if timer.ticks() - ms_passed > 500 {
            blink = !blink;
            ms_passed = timer.ticks();
        }

        let _ = text.renderer.set_scale(SCREEN_SCALE, SCREEN_SCALE);
        let _ = text.renderer.set_draw_color(colors::LIGHT_BLUE);
        let _ = text.renderer.clear();

        let _ = text.renderer.set_draw_color(colors::BLUE);

        let screen_rect = Rect::new(SCREEN_X, SCREEN_Y, SCREEN_WIDTH, SCREEN_HEIGHT);
        let _ = text.renderer.fill_rect(screen_rect);

        let _ = text.render(blink);
        let _ = text.renderer.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
