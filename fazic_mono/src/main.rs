extern crate sdl2;

pub mod ast;
pub mod runtime;
pub mod parser;
pub mod sdl_screen;
pub mod emscripten;

fn main() {
    sdl_screen::main();
}
