extern crate sdl2;
extern crate rand;

//pub mod ast;
//pub mod runtime;
//pub mod parser;
//pub mod sdl_screen;

pub mod emscripten;
pub mod fazic;

use std::{process};
use sdl2::event::{Event};
use sdl2::keyboard::*;
use sdl2::render;
use sdl2::pixels::PixelFormatEnum;

const SCALE: usize = 2;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("fazic", (fazic::screen::WIDTH * SCALE) as u32, (fazic::screen::HEIGHT * SCALE) as u32)
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

    let mut fps = ctx.timer().unwrap();
    let mut fps_update = ctx.timer().unwrap();
    let mut fps_frames = 0;
    let mut fps_update_ms = 0;

    let mut fazic = fazic::Fazic::new();

    let mut rgb_pixels: Box<[u8; fazic::screen::PIXELS * 3]> = Box::new([0; fazic::screen::PIXELS * 3]);

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    // screen.putpixel(
                    //     rand::random::<u8>() as usize,
                    //     rand::random::<u8>() as usize,
                    //     1
                    // );
                    fazic.screen.set_current_color(6);
                    fazic.screen.clear();
                    fazic.screen.set_current_color(14);
                    fazic.screen.print("Xsdfsdf".to_string(),
                        rand::random::<u8>() as usize,
                        rand::random::<u8>() as usize,
                        14
                    );


                        // let v = if screen.pixels[0] == 1 { 0 } else { 1 };
                    // for i in 0..SCREEN_PIXELS {
                    //     screen.pixels[i] = v;
                    // }
                },
                 _ => ()
            }
        }

        for i in 0..fazic::screen::PIXELS {
            match fazic.screen.pixels[i] {
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
                2 => {
                    rgb_pixels[i*3]   = 224;
                    rgb_pixels[i*3+1] = 64;
                    rgb_pixels[i*3+2] = 64;
                },
                3 => {
                    rgb_pixels[i*3]   = 96;
                    rgb_pixels[i*3+1] = 255;
                    rgb_pixels[i*3+2] = 255;
                },
                4 => {
                    rgb_pixels[i*3]   = 224;
                    rgb_pixels[i*3+1] = 94;
                    rgb_pixels[i*3+2] = 224;
                },
                5 => {
                    rgb_pixels[i*3]   = 64;
                    rgb_pixels[i*3+1] = 224;
                    rgb_pixels[i*3+2] = 64;
                },
                6 => {
                    rgb_pixels[i*3]   = 64;
                    rgb_pixels[i*3+1] = 64;
                    rgb_pixels[i*3+2] = 224;
                },
                7 => {
                    rgb_pixels[i*3]   = 255;
                    rgb_pixels[i*3+1] = 255;
                    rgb_pixels[i*3+2] = 64;
                },
                8 => {
                    rgb_pixels[i*3]   = 224;
                    rgb_pixels[i*3+1] = 160;
                    rgb_pixels[i*3+2] = 64;
                },
                9 => {
                    rgb_pixels[i*3]   = 156;
                    rgb_pixels[i*3+1] = 116;
                    rgb_pixels[i*3+2] = 72;
                },
                10 => {
                    rgb_pixels[i*3]   = 255;
                    rgb_pixels[i*3+1] = 160;
                    rgb_pixels[i*3+2] = 160;
                },
                11 => {
                    rgb_pixels[i*3]   = 84;
                    rgb_pixels[i*3+1] = 84;
                    rgb_pixels[i*3+2] = 84;
                },
                12 => {
                    rgb_pixels[i*3]   = 136;
                    rgb_pixels[i*3+1] = 136;
                    rgb_pixels[i*3+2] = 136;
                },
                13 => {
                    rgb_pixels[i*3]   = 160;
                    rgb_pixels[i*3+1] = 255;
                    rgb_pixels[i*3+2] = 160;
                },
                14 => {
                    rgb_pixels[i*3]   = 160;
                    rgb_pixels[i*3+1] = 160;
                    rgb_pixels[i*3+2] = 255;
                },
                15 => {
                    rgb_pixels[i*3]   = 192;
                    rgb_pixels[i*3+1] = 192;
                    rgb_pixels[i*3+2] = 192;
                },
                _ => unreachable!()
            }
        }

        texture.update(None,
                       &mut *rgb_pixels,
                       fazic::screen::WIDTH * 3).unwrap();

        let _ = renderer.copy(&texture, None, None);
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
