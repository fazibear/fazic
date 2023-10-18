extern crate console_error_panic_hook;
extern crate fazic;

mod utils;
mod canvas;

// use fazic::file_system::FileSystem;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::window;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init().expect("error initializing log");
    
    let mut fazic = fazic::Fazic::new();
    let context = canvas::Canvas::new();
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());
        
        fazic.tick();

        todo!();
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
