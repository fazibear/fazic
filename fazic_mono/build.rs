extern crate peg;

fn main() {
    peg::cargo_build("src/fazic/parser.rustpeg");
    rustc_link_arg("-L");
    rustc_link_arg("/usr/local/Cellar/sdl2/2.26.5/lib");
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

fn rustc_link_arg(arg: &str) {
    println!("cargo:rustc-link-arg={}", arg);
}
