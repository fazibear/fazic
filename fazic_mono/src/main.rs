extern crate sdl2;

pub mod ast;
pub mod node_builder;
pub mod runtime;
pub mod parser;
pub mod grammar;
pub mod screen;

fn main() {
    screen::main();
}
