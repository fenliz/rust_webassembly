#![feature(wasm_import_memory)]
#![wasm_import_memory]

use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[no_mangle]
pub fn test(x: i32) {
    unsafe { javascript_fn(x); }
}
extern "C" {
    pub fn javascript_fn(num: i32);
}

#[no_mangle]
pub fn test_string(x: i32) {
    unsafe {
        let msg = format!("Hello World: {}", x);
        javascript_string_fn(msg.as_ptr(), msg.len() as u32);
    }
}
extern "C" {
    pub fn javascript_string_fn(ptr: *const u8, len: u32);
}

#[no_mangle]
pub extern "C" fn test_string_return(data: *mut c_char) -> *mut c_char {
    data
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
