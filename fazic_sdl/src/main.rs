#![feature(link_args)]

//#![feature(alloc_system)]
//extern crate alloc_system;

#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate rand;
extern crate sdl2;

extern crate fazic;

use fazic::config::*;
use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::process;

const SCALE: u32 = 1;
const WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE;
const HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE;

pub fn main() {
    #[cfg(debug_assertions)]
    simple_logger::SimpleLogger::new().init().unwrap();

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = video_ctx
        .window("fazic", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let mut events = ctx.event_pump().unwrap();

    let mut timer = ctx.timer().unwrap();

    let mut fps_last_time = 0;

    let mut fps = 0;
    let mut tps = 0;

    let mut fazic = fazic::Fazic::new();

    let draw_callback = move |action| {
        match action {
            fazic::DrawAction::PutPixel(x, y, r, g, b) => {
                //debug!("pix: {} {} {} {} {}", x, y, r, g, b);
                canvas.set_draw_color(Color::RGB(r,g,b));
                let _ = canvas.draw_point(Point::new(x,y));
            }
            fazic::DrawAction::Redraw() => {
                canvas.present();
            }
            fazic::DrawAction::Clear(r, g, b) => {
                canvas.set_draw_color(Color::RGB(r,g,b));
                canvas.clear();
            }
        };
    };

    fazic.set_draw_callback(Box::new(draw_callback));

    loop {
        let main_loop_time = timer.ticks();

        if fps % 5 == 0 {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => process::exit(1),
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => fazic.stop_key(),
                    Event::KeyDown {
                        keycode: Some(key),
                        keymod: Mod::LGUIMOD,
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(key),
                        keymod: Mod::RGUIMOD,
                        ..
                    } => match key {
                        Keycode::Num1 => fazic.set_current_text_color(0),
                        Keycode::Num2 => fazic.set_current_text_color(1),
                        Keycode::Num3 => fazic.set_current_text_color(2),
                        Keycode::Num4 => fazic.set_current_text_color(3),
                        Keycode::Num5 => fazic.set_current_text_color(4),
                        Keycode::Num6 => fazic.set_current_text_color(5),
                        Keycode::Num7 => fazic.set_current_text_color(6),
                        Keycode::Num8 => fazic.set_current_text_color(7),
                        _ => (),
                    },
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Left => fazic.left_key(),
                        Keycode::Right => fazic.right_key(),
                        Keycode::Up => fazic.up_key(),
                        Keycode::Down => fazic.down_key(),
                        Keycode::Backspace => fazic.backspace_key(),
                        Keycode::Return => fazic.enter_key(),
                        _ => (),
                    },
                    Event::TextInput { text: string, .. } => fazic.insert_string(string),
                    _ => (),
                }
            }
        }

        if fps == 1 || fps == 30 {
            fazic.blink_cursor();
        }

        if timer.ticks() - fps_last_time > 1000 {
            debug!("FPS: {}", fps);
            fps_last_time = timer.ticks();
            fps = 0;

            debug!("TPS: {}", tps);
            tps = 0;
        }
        fps += 1;

        while timer.ticks() - main_loop_time < 16 {
            tps += 1;
            fazic.tick();
        }
    }
}
