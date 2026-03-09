use std::ffi::{CStr, c_char, c_int};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// # Safety
///
/// Called from C. `msg` must be a null-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_print(msg: *const c_char, is_stderr: c_int) {
    if msg.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(msg) };
    let s = c_str.to_string_lossy();
    let s = s.trim_end();

    if is_stderr != 0 {
        log::warn!("[REDIRECTED] [STDERR] {}", s);
    } else {
        log::info!("[REDIRECTED] [STDOUT] {}", s);
    }
}

fn main() {
    env_logger::init();

    unsafe {
        ffi::c_hello();
        ffi::c_error();
    }
}

// Manual definition if not using `bindgen`
// (Alternative to the `include!` macro above)
//
// unsafe extern "C" {
//     pub fn c_hello();
//     pub fn c_error();
// }
