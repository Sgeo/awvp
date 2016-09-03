use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

#[macro_use]
extern crate log;

mod rc;

use rc::rc;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INSTANCE: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
}

fn instance() -> *mut () {
    INSTANCE.load(Ordering::SeqCst)
}

pub mod unimpl;

#[allow(dead_code)]
mod raw;
mod globals;
mod instance;
mod attributes;

pub mod impled;

#[allow(dead_code)]
fn spare() { println!(""); } //adding this (doesn't have to be named "spare") makes the compilation work. https://github.com/rust-lang/rust/issues/18807

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
