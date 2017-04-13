extern crate sdl2;
extern crate rand;

pub mod emscripten;
pub mod fazic;

use std::{process};
use sdl2::event::{Event};
use sdl2::keyboard::*;
use sdl2::render;
use sdl2::pixels::PixelFormatEnum;

const SCALE: u16 = 2;
const WIDTH: u32 = (fazic::screen::WIDTH * SCALE) as u32;
const HEIGHT: u32 = (fazic::screen::HEIGHT * SCALE) as u32;
const RGB_WIDTH: usize = fazic::screen::WIDTH as usize * 3;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("fazic", WIDTH, HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut renderer = match window
        .renderer()
        .accelerated()
        .build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

    let mut texture = renderer.create_texture(PixelFormatEnum::RGB24,
                                              render::TextureAccess::Streaming,
                                              fazic::screen::WIDTH as u32,
                                              fazic::screen::HEIGHT as u32
                                             ).unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut timer = ctx.timer().unwrap();

    let mut fps_last_tick = 0;
    let mut fps_frames = 0;
    let mut blink_last_tick = 0;

    let mut fazic = fazic::Fazic::new();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(key), keymod: LGUIMOD, ..} |
                Event::KeyDown { keycode: Some(key), keymod: RGUIMOD, ..} => {
                    match key {
                        Keycode::Num1 => fazic.text.set_current_color(0),
                        Keycode::Num2 => fazic.text.set_current_color(1),
                        Keycode::Num3 => fazic.text.set_current_color(2),
                        Keycode::Num4 => fazic.text.set_current_color(3),
                        Keycode::Num5 => fazic.text.set_current_color(4),
                        Keycode::Num6 => fazic.text.set_current_color(5),
                        Keycode::Num7 => fazic.text.set_current_color(6),
                        Keycode::Num8 => fazic.text.set_current_color(7),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    match key {
                        Keycode::Left => fazic.text.left(),
                        Keycode::Right => fazic.text.right(),
                        Keycode::Up => fazic.text.up(),
                        Keycode::Down => fazic.text.down(),
                        Keycode::Backspace => fazic.text.backspace(),
                        Keycode::Return => fazic.text.enter(),
                        _ => (),
                    }
                },
                Event::TextInput { text: string, ..} => fazic.text.insert_string(string),
                _ => ()
            }
        }

        if timer.ticks() - blink_last_tick > 250 {
            fazic.text.blink_cursor();
            blink_last_tick = timer.ticks();
        }

        fazic.tick();

        texture.update(None,
                       &mut *fazic.screen.rgb_pixels,
                       RGB_WIDTH).unwrap();

        let _ = renderer.copy(&texture, None, None);
        renderer.present();

        if timer.ticks() - fps_last_tick > 1000 {
            //println!("FPS: {}", fps_frames);
            fps_last_tick = timer.ticks();
            fps_frames = 0;
        }
        fps_frames = fps_frames + 1;
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
