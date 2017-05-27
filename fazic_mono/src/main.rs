#![feature(link_args)]

extern crate sdl2;
extern crate rand;

pub mod targets;
pub mod fazic;

#[cfg(test)]
pub mod tests;


use std::{process};
use sdl2::event::{Event};
use sdl2::keyboard::*;
use sdl2::pixels::PixelFormatEnum;

const SCALE: u16 = 2;
const WIDTH: u32 = (fazic::SCREEN_WIDTH * SCALE) as u32;
const HEIGHT: u32 = (fazic::SCREEN_HEIGHT * SCALE) as u32;
const RGB_WIDTH: usize = fazic::SCREEN_WIDTH as usize * 3;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();


    let window = video_ctx
        .window("fazic", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

     let mut texture = texture_creator.create_texture_streaming(
         PixelFormatEnum::RGB24,
         fazic::SCREEN_WIDTH as u32,
         fazic::SCREEN_HEIGHT as u32
    ).unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut timer = ctx.timer().unwrap();

    let mut fps_last_tick = 0;
    let mut fps_frames = 0;
    let mut blink_last_tick = 0;

    let mut fazic = fazic::Fazic::new();

    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => process::exit(1),
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => fazic.stop_key(),
                Event::KeyDown { keycode: Some(key), keymod: LGUIMOD, ..} |
                Event::KeyDown { keycode: Some(key), keymod: RGUIMOD, ..} => {
                    match key {
                        Keycode::Num1 => fazic.set_current_text_color(0),
                        Keycode::Num2 => fazic.set_current_text_color(1),
                        Keycode::Num3 => fazic.set_current_text_color(2),
                        Keycode::Num4 => fazic.set_current_text_color(3),
                        Keycode::Num5 => fazic.set_current_text_color(4),
                        Keycode::Num6 => fazic.set_current_text_color(5),
                        Keycode::Num7 => fazic.set_current_text_color(6),
                        Keycode::Num8 => fazic.set_current_text_color(7),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    match key {
                        Keycode::Left => fazic.left_key(),
                        Keycode::Right => fazic.right_key(),
                        Keycode::Up => fazic.up_key(),
                        Keycode::Down => fazic.down_key(),
                        Keycode::Backspace => fazic.backspace_key(),
                        Keycode::Return => {
                            fazic.enter_key()
                        },
                        _ => (),
                    }
                },
                Event::TextInput { text: string, ..} => fazic.insert_string(string),
                _ => ()
            }
        }

        if timer.ticks() - blink_last_tick > 250 {
            fazic.blink_cursor();
            blink_last_tick = timer.ticks();
        }

        fazic.tick();

        if fazic.redraw() {
            texture.update(None,
                           fazic.get_rgb_pixels(),
                           RGB_WIDTH).unwrap();

            let _ = canvas.copy(&texture, None, None);
            canvas.present();
            fazic.redrawed();
        }

        if timer.ticks() - fps_last_tick > 1000 {
            println!("FPS: {}", fps_frames);
            fps_last_tick = timer.ticks();
            fps_frames = 0;
        }
        fps_frames = fps_frames + 1;
    };

    targets::set_main_loop_callback(main_loop);
}
