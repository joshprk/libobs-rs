use std::{ffi::CString, ptr};


fn main() {
    let locale = CString::new("en-US").unwrap();
    println!("Locale pointer: {:?}", locale.as_ptr());

    let startup_result = unsafe {
        libobs::obs_startup(locale.as_ptr(), ptr::null(), ptr::null_mut())
    };
    if !startup_result {
        panic!("error on libobs startup");
    }
    println!("OBS startup successful");

    unsafe {
        libobs::obs_shutdown()
    };
}
