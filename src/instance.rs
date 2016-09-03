use std::os::raw::{c_int, c_char, c_void};

use raw::{aw, vp};

use rc::rc;

use attributes::AttribBuffer;

use globals::GLOBALS;

pub struct Instance {
    pub vp: vp::VPInstance,
    pub attributes: AttribBuffer
}

unsafe impl Send for Instance {}

#[no_mangle]
pub extern fn aw_init() -> c_int {
    extern crate flexi_logger;
    let mut config = flexi_logger::LogConfig::new();
    config.log_to_file = true;
    flexi_logger::init(config, None);
    
    debug!("instance's aw_init!");
    rc(unsafe { vp::init(3) })
}

#[no_mangle]
pub extern fn aw_create(domain: *const c_char, port: c_int, instance: *mut *mut c_void) -> ::std::os::raw::c_int {
    let vp;
    let result;
    unsafe {
        vp = vp::create();
        debug!("vp pointer: {:?}", vp);
        *instance = vp;
        result = vp::connect_universe(vp, domain, port);
    }
    let instance = Instance { vp: vp, attributes: AttribBuffer::new() };
    let mut globals = GLOBALS.lock().unwrap();
    globals.current = vp as usize;
    globals.instances.insert(vp as usize, instance);
    rc(result)
 }