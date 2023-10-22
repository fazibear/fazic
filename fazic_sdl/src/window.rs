use fazic::config::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::image::LoadTexture;

const RGB_WIDTH: usize = SCREEN_WIDTH as usize * 3;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const WINDOW_WIDTH: u32 = 1140;
const WINDOW_HEIGHT: u32 = 698;

pub struct Window {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        let video_ctx = ctx.video().unwrap();

        let window = video_ctx
            .window("fazic", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

//        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        Self { canvas }
    }

    pub fn update(&mut self, pixels: &[u8]) {
        let texture_creator = self.canvas.texture_creator();
        
        let tv_path = std::path::Path::new("tv.jpg");
        let tv_texture = texture_creator.load_texture(&tv_path).unwrap();
        
        self.canvas.copy(&tv_texture, None, None).unwrap();
        
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();
        
        let mut rgb = Vec::with_capacity(pixels.len() * 3);

        for color in pixels {
            let (r, g, b) = fazic::colors::rgb_for(color);
            rgb.push(r);
            rgb.push(g);
            rgb.push(b);
        }

        texture.update(None, rgb.as_slice(), RGB_WIDTH).unwrap();
        self.canvas.copy_ex(
            &texture,
            None,
            Some(sdl2::rect::Rect::new(165, 110, WIDTH, HEIGHT)),
            0.0,
            None,
            false,
            false,
        ).unwrap();
        self.canvas.present();
    }
}
