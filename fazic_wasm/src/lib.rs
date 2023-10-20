extern crate fazic;
extern crate console_error_panic_hook;


mod utils;
mod canvas;
mod body;

// use fazic::file_system::FileSystem;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::window;
use log::*;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// ty nie chcesz byc z kims kto sie czepia
/// ja nie chce bym z kims komu jestem obojetna

/// jak juz cos chce to jest problem

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    debug!("start...");
    
    let mut fazic = fazic::Fazic::new();

    let mut body = Box::new(body::Body::new());
    
    body.attach_events();

    let context = canvas::Canvas::new();
    let perf = window().unwrap().performance().unwrap();
  
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut fps_last_time = perf.now();
    
    let mut fps = 0;
    let mut tps = 0;

    *g.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());
        
        //
        // main loop
        // 
        
        let main_loop_time = perf.now();
        
        if fps % 5 == 0 {
           // events.process(&mut fazic);
        }

        if fps == 1 || fps == 30 {
            fazic.blink_cursor();
        }
        
        if fazic.need_to_redraw() {
            context.draw(&fazic.get_pixels());
        }
        
        if perf.now() - fps_last_time > 1000.0 {
            debug!("FPS: {}", fps);
            fps_last_time = perf.now();
            fps = 0;

            debug!("TPS: {}", tps);
            tps = 0;
        }

        fps += 1;

        while perf.now() - main_loop_time < 16.0 {
            tps += 1;
            if fazic.tick() {
                break;
            };
        }

        //
        // end main loop
        // 

    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
