#![allow(non_snake_case)]

extern crate libc;

#[no_mangle]
pub extern fn aw_init(version: libc::c_int) -> libc::c_int {
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
