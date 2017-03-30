extern crate sdl2;

pub mod ast;
pub mod runtime;
pub mod parser;
pub mod screen;
pub mod emscripten;

fn main() {
    screen::main();
}
