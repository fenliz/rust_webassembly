//#![feature(wasm_import_memory)]
//#![wasm_import_memory]
//
//use std::mem;
//use std::ffi::CString;
//use std::os::raw::{c_char, c_void};
//
//#[no_mangle]
//pub fn add_one(x: i32) -> i32 {
//    x + 1
//}
//
//#[no_mangle]
//pub fn test(x: i32) {
//    unsafe { javascript_fn(x); }
//}
//extern "C" {
//    pub fn javascript_fn(num: i32);
//}
//
//#[no_mangle]
//pub fn test_string(x: i32) {
//    unsafe {
//        let msg = format!("Hello World: {}", x);
//        javascript_string_fn(msg.as_ptr(), msg.len() as u32);
//    }
//}
//extern "C" {
//    pub fn javascript_string_fn(ptr: *const u8, len: u32);
//}
//
//#[no_mangle]
//pub extern "C" fn test_string_return(data: *mut c_char) -> *mut c_char {
//    data
//}
//
//#[no_mangle]
//pub extern "C" fn alloc(size: usize) -> *mut c_void {
//    let mut buf = Vec::with_capacity(size);
//    let ptr = buf.as_mut_ptr();
//    mem::forget(buf);
//    return ptr as *mut c_void;
//}
//
//#[no_mangle]
//pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
//    unsafe  {
//        let _buf = Vec::from_raw_parts(ptr, 0, cap);
//    }
//}
//
//#[no_mangle]
//pub extern "C" fn dealloc_str(ptr: *mut c_char) {
//    unsafe {
//        let _ = CString::from_raw(ptr);
//    }
//}

#[macro_use]
extern crate stdweb;
extern crate glenum;

use std::cell::RefCell;
use std::rc::Rc;
use glenum::*;
use stdweb::web::{document, window, Element};
use stdweb::unstable::TryInto;

pub struct WebGL2RenderingContext {
    reference: stdweb::Reference,
}
impl WebGL2RenderingContext {
    pub fn new(canvas: &Element) -> WebGL2RenderingContext {
        let gl = js! { return (@{canvas}).getContext("webgl2"); };

        WebGL2RenderingContext {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        js! {
            (@{&self.reference}).clearColor(@{r}, @{g}, @{b}, @{a});
        }
    }

    pub fn clear(&self, bit: BufferBit) {
        js! {
            (@{&self.reference}).clear(@{bit as i32});
        }
    }
}

pub struct App {
    gl: WebGL2RenderingContext,
    last_time: i64,
}

impl App {
    pub fn new(canvas: &Element) -> App {
        let gl = WebGL2RenderingContext::new(&canvas);

        App {
            gl: gl,
            last_time: now(),
        }
    }

    pub fn init(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn render(&mut self) {
        let cur_time = now();

        if cur_time - self.last_time > 1000 {
            self.gl
                .clear_color(rand() as f32, rand() as f32, rand() as f32, 1.0);
            self.gl.clear(BufferBit::Color);

            self.last_time = cur_time;
        }
    }
}

fn rand() -> f64 {
    let value = js! {
        return Math.random();
    };

    value.try_into().unwrap()
}

fn now() -> i64 {
    let value = js! {
        return Date.now();
    };

    value.try_into().unwrap()
}

fn main_loop(app: Rc<RefCell<App>>) {
    app.borrow_mut().render();

    window().request_animation_frame(move |_| {
        main_loop(app);
    });
}

fn main() {
    stdweb::initialize();
    let canvas = document().get_element_by_id("canvas").unwrap();
    let app = Rc::new(RefCell::new(App::new(&canvas)));

    app.borrow_mut().init();

    window().request_animation_frame(move |_| {
        main_loop(app);
    });

    stdweb::event_loop();
}