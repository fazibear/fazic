/* emscripten */

#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "emscripten")]
pub use self::emscripten::*;

/* other */

#[cfg(not(target_os = "emscripten"))]
pub mod other;

#[cfg(not(target_os = "emscripten"))]
pub use self::other::*;
