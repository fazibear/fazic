mod utils;

use fazic::config::*;
use fazic::file_system::FileSystem;
use fazic::*;
use log::*;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData, KeyboardEvent, Performance, Storage};

struct StorageFileSystem {
    storage: Storage,
}

impl StorageFileSystem {
    pub fn new(storage: Storage) -> StorageFileSystem {
        StorageFileSystem { storage }
    }
}

impl FileSystem for StorageFileSystem {
    fn dir(&self) -> Result<Vec<String>, String> {
        let mut dir = vec![];
        let len = self.storage.length().unwrap();
        for i in 0..len {
            let item = self.storage.key(i).unwrap().unwrap();
            dir.push(item);
        }
        Ok(dir)
    }

    fn load(&self, filename: &str) -> Result<String, String> {
        if let Ok(Some(item)) = self.storage.get_item(filename) {
            return Ok(item);
        } else {
            return Err(format!("{} not found", filename));
        }
    }

    fn save(&mut self, filename: &str, program: String) -> Result<(), String> {
        self.storage.set_item(filename, program.as_str()).unwrap();
        Ok(())
    }
}

#[wasm_bindgen]
pub fn initialise() {
    utils::set_panic_hook();
    console_log::init_with_level(Level::Debug).expect("error initializing log");
}

#[wasm_bindgen]
pub struct JSFazic {
    fazic: Fazic,
    ctx: Option<CanvasRenderingContext2d>,
    perf: Option<Performance>,
    fps_last_time: f64,
    main_loop_time: f64,
    fps: u32,
    tps: u32,
}

#[wasm_bindgen]
impl JSFazic {
    pub fn new() -> JSFazic {
        JSFazic {
            fazic: Fazic::new(),
            ctx: None,
            perf: None,
            fps_last_time: 0.0,
            main_loop_time: 0.0,
            fps: 0,
            tps: 0,
        }
    }

    pub fn set_ctx(&mut self, ctx: CanvasRenderingContext2d) {
        self.ctx = Some(ctx);
    }

    fn ctx(&self) -> &CanvasRenderingContext2d {
        self.ctx.as_ref().expect("ctx is not set")
    }

    pub fn set_perf(&mut self, perf: Performance) {
        self.perf = Some(perf);
    }

    fn perf(&self) -> &Performance {
        self.perf.as_ref().expect("perf is not set")
    }

    pub fn set_storage(&mut self, storage: Storage) {
        self.fazic.set_file_system(Box::new(StorageFileSystem::new(storage)));
    }

    pub fn on_keydown(&mut self, event: KeyboardEvent) {
        if event.key() == "ArrowUp" {
            self.fazic.up_key();
        }
        if event.key() == "ArrowDown" {
            self.fazic.down_key();
        }
        if event.key() == "ArrowLeft" {
            self.fazic.left_key();
        }
        if event.key() == "ArrowRight" {
            self.fazic.right_key();
        }
        if event.key() == "Enter" {
            self.fazic.enter_key();
        }
        if event.key() == "Backspace" {
            self.fazic.backspace_key();
        }
        if event.key() == "Escape" {
            self.fazic.stop_key();
        }
        if event.key().len() == 1 {
            self.fazic.insert_string(event.key())
        }
        debug!("on_key_down: {:?}", event);
    }

    pub fn animation_loop(&mut self) {
        self.main_loop_time = self.perf().now();

        if self.fps == 1 || self.fps == 30 {
            self.fazic.blink_cursor();
        }

        if self.fazic.need_to_redraw() {
            let pixels = self.fazic.get_pixels();
            let mut rgba = Vec::with_capacity(pixels.len() * 4);

            for color in pixels {
                let (r, g, b) = fazic::colors::rgb_for(color);
                rgba.push(r);
                rgba.push(g);
                rgba.push(b);
                rgba.push(255);
            }

            let image_data =
                ImageData::new_with_u8_clamped_array(Clamped(rgba.as_slice()), SCREEN_WIDTH as u32)
                    .unwrap();

            self.ctx().put_image_data(&image_data, 0.0, 0.0).unwrap();
        }

        if self.perf().now() - self.fps_last_time > 1000.0 {
            debug!("FPS: {}", self.fps);
            self.fps_last_time = self.perf().now();
            self.fps = 0;

            debug!("TPS: {}", self.tps);
            self.tps = 0;
        }

        self.fps += 1;

        while self.perf().now() - self.main_loop_time < 16.0 {
            self.tps += 1;
            if self.fazic.tick() {
                break;
            };
        }
    }
}
