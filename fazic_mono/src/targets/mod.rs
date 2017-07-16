/* emscripten */

#[cfg(target_os = "emscripten")]
mod emscripten;

#[cfg(target_os = "emscripten")]
pub use self::emscripten::*;

/* other */

#[cfg(not(target_os = "emscripten"))]
mod other;

#[cfg(not(target_os = "emscripten"))]
pub use self::other::*;
