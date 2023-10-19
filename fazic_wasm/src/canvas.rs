use fazic::config::*;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

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
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(SCREEN_WIDTH as u32);
        canvas.set_height(SCREEN_HEIGHT as u32);

        let context = canvas
            .get_context_with_context_options("2d", &context_attributes)
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { context }
    }

    pub fn draw(&self, rgb: &[u8]) {
        let image_data =
            ImageData::new_with_u8_clamped_array(Clamped(rgb), SCREEN_WIDTH as u32).unwrap();

        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}
