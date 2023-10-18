use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use fazic::config::*;

const SCALE: f32 = 2.0;
const WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE as u32;
const HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE as u32;

#[derive(Debug)]
pub struct Canvas {
    pub context: CanvasRenderingContext2d,
}
impl Canvas {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();

        let mut context_attributes = web_sys::ContextAttributes2d::new();
        context_attributes.will_read_frequently(true);

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(WIDTH.into());
        canvas.set_height(HEIGHT.into());

        let context = canvas
            .get_context_with_context_options("2d", &context_attributes)
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { context }
    }
}
