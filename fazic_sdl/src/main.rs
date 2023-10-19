#[macro_use]
extern crate log;
extern crate fazic;
extern crate rand;
extern crate sdl2;
extern crate simple_logger;

mod events;
mod file_system;
mod window;
mod instant_rtc;

pub fn main() {
    #[cfg(debug_assertions)]
    simple_logger::SimpleLogger::new().init().unwrap();

    let ctx = sdl2::init().unwrap();

    let mut window = window::Window::new(&ctx);
    let mut events = events::Events::new(&ctx);

    let timer = ctx.timer().unwrap();

    let mut fps_last_time = 0;

    let mut fps = 0;
    let mut tps = 0;

    let mut fazic = fazic::Fazic::new();

    fazic.set_file_system(Box::new(file_system::DiskFileSystem::new()));
    fazic.set_rtc(Box::new(instant_rtc::InstantRtc::new()));

    loop {
        let main_loop_time = timer.ticks();

        if fps % 5 == 0 {
            events.process(&mut fazic);
        }

        if fps == 1 || fps == 30 {
            fazic.blink_cursor();
        }

        if fazic.need_to_redraw() {
            window.update(fazic.get_rgb_pixels());
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
            if fazic.tick() {
                break;
            };
        }
    }
}
