use sdl2::event::Event;
use sdl2::keyboard::*;

use std::process;

pub struct Events {
    event_pump: sdl2::EventPump,
}

impl Events {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        Self {
            event_pump: ctx.event_pump().unwrap(),
        }
    }

    pub fn process(&mut self, fazic: &mut ::fazic::Fazic) {
        for event in self.event_pump.poll_iter() {
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
}
