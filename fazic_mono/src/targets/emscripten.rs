#[link_args = "-g0 -Oz --closure 1 --llvm-lto 1 --memory-init-file 0 -s USE_SDL=2 --js-library src/targets/emscripten.js"]
extern {}

use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_char};
use std::ffi::CString;

const HOST: &str = "http://localhost:8080";

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();

const HOST: &str = "http://localhost:8080";

extern {
    fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
    fn js_fetch(url: *const c_char, method: *const c_char, data: *const c_char, code: *const c_int, resp: *const c_char, size: *const c_int) -> *const c_char;
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F: FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

pub fn load(name: &String) -> Result<String, String> {
    let (code, resp) = fetch(name, "get", &"".to_string());
    match code {
        200 => Ok(resp),
        _ => Err(resp),
    }
}


pub fn save(name: &String, body: &String) -> Result<String, String> {
    let (code, resp) = fetch(name, "post", body);
    match code {
        200 => Ok(resp),
        _ => Err(resp),
    }
}

fn fetch(name: &String, method: &str, data: &String) -> (i32, String) {
    let resp: String;
    let code: c_int = 0;

    let data = CString::new(
        format!("{}", data)
    ).unwrap();

    let url = CString::new(
        format!("{}/file/{}", HOST, name)
    ).unwrap();

    let method = CString::new(method).unwrap();

    unsafe {
        let mut resp_p = Vec::with_capacity(1024);
        let size_p: c_int = 0;

        js_fetch(url.as_ptr(), method.as_ptr(), data.as_ptr(), &code, resp_p.as_ptr(), &size_p);

        resp_p.set_len(size_p as usize - 1);

        resp = CString::from_vec_unchecked(resp_p)
            .to_string_lossy()
            .into_owned();
    }

    (code, resp)
}
