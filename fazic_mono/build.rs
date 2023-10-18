extern crate peg;

fn main() {
    rustc_link_arg("-L");
    rustc_link_arg("/opt/homebrew/Cellar/sdl2/2.28.4/lib");
    rustc_link_arg("-O3");
    rustc_link_arg("--llvm-opts");
    rustc_link_arg("2");
    rustc_link_arg("--memory-init-file");
    rustc_link_arg("0");
    rustc_link_arg("-s");
    rustc_link_arg("USE_SDL=2");
    rustc_link_arg("--js-library");
    rustc_link_arg("src/targets/emscripten.js");
}

fn rustc_link_arg(_arg: &str) {
    //    println!("cargo:rustc-link-arg={}", arg);
}
