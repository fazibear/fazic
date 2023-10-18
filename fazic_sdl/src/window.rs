use fazic::config::*;
use sdl2::pixels::PixelFormatEnum;

const SCALE: f32 = 2.0;
const WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE as u32;
const HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE as u32;
const RGB_WIDTH: usize = SCREEN_WIDTH as usize * 3;

pub struct Window {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        let video_ctx = ctx.video().unwrap();

        let window = video_ctx
            .window("fazic", WIDTH, HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
        }
    }

    pub fn update(&mut self, rgb: &[u8]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        texture.update(None, rgb, RGB_WIDTH).unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
