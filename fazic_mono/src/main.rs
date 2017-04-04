extern crate sdl2;
extern crate rand;

//pub mod ast;
//pub mod runtime;
//pub mod parser;
//pub mod sdl_screen;
//pub mod emscripten;

pub mod screen;

use screen::*;
use std::{process};
use sdl2::event::{Event};
use sdl2::keyboard::*;
use sdl2::render;
use sdl2::pixels::PixelFormatEnum;


pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("fazic", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
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

    let mut texture = renderer.create_texture(PixelFormatEnum::BGR24,
                                              render::TextureAccess::Streaming,
                                              SCREEN_WIDTH as u32,
                                              SCREEN_HEIGHT as u32
                                             ).unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut fps = ctx.timer().unwrap();
    let mut fps_update = ctx.timer().unwrap();
    let mut fps_frames = 0;
    let mut fps_update_ms = 0;

    let mut screen = Screen::new();

    let mut rgb_pixels: Box<[u8; SCREEN_PIXELS * 3]> = Box::new([0; SCREEN_PIXELS * 3]);

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    screen.putpixel(
                        rand::random::<u8>() as usize,
                        rand::random::<u8>() as usize,
                        1
                    )
                    // let v = if screen.pixels[0] == 1 { 0 } else { 1 };
                    // for i in 0..SCREEN_PIXELS {
                    //     screen.pixels[i] = v;
                    // }
                },
                 _ => ()
            }
        }

        for i in 0..SCREEN_PIXELS {
            match screen.pixels[i] {
                0 => {
                    rgb_pixels[i*3]   = 0;
                    rgb_pixels[i*3+1] = 0;
                    rgb_pixels[i*3+2] = 0;
                },
                1 => {
                    rgb_pixels[i*3]   = 255;
                    rgb_pixels[i*3+1] = 255;
                    rgb_pixels[i*3+2] = 255;
                },
                _ => unreachable!()
            }
        }

        texture.update(None,
                       &mut *rgb_pixels,
                       SCREEN_WIDTH * 3).unwrap();

        renderer.copy(&texture, None, None);
        renderer.present();

        if fps_update.ticks() - fps_update_ms > 1000 {
            println!("FPS: {}", fps_frames / (fps.ticks() / 1000));
            fps_update_ms = fps_update.ticks();
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
