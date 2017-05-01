pub fn set_main_loop_callback<F>(mut f: F) where F: FnMut() {
    loop {
        f();
    }
}
