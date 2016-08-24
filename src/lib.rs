use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

extern crate libc;
use libc::{c_int, c_char};

#[link(name="VPSDK")]
extern {
    fn vp_init(version: c_int) -> c_int;
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INSTANCE: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
}

fn instance() -> *mut () {
    INSTANCE.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern fn aw_init(_version: c_int) -> c_int {
    unsafe {
        vp_init(3);
    }
    0
}

#[no_mangle]
pub extern fn aw_say(_text: *const c_char) -> c_int {
    println!("Say!");
    0
}

#[no_mangle]
pub extern fn aw_server_world_set(_foo: c_int) -> c_int {
    0
}

#[allow(dead_code)]
fn spare() { println!(""); } //adding this (doesn't have to be named "spare") makes the compilation work. https://github.com/rust-lang/rust/issues/18807

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
