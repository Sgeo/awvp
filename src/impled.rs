use std::os::raw::{c_int, c_char, c_void};
use std::ffi::CStr;

use globals::{GLOBALS, vp};

use raw::{aw, vp};

use instance::Instance;
use attributes::AttribBuffer;

use rc::rc;

#[no_mangle]
pub extern fn aw_init() -> c_int {
    extern crate flexi_logger;
    let mut config = flexi_logger::LogConfig::new();
    config.log_to_file = true;
    flexi_logger::init(config, None).expect("Unable to initialize logger");
    
    debug!("instance's aw_init!");
    rc(unsafe { vp::init(3) })
}

#[no_mangle]
pub extern fn aw_create(domain: *const c_char, port: c_int, instance: *mut *mut c_void) -> ::std::os::raw::c_int {
    let vp;
    let result;
    unsafe {
        let dest_domain = if domain.is_null() {
            CStr::from_bytes_with_nul_unchecked(b"universe.virtualparadise.org\0").as_ptr()
        } else {
            domain
        };
        let dest_port = if port == 0 { 57000 } else { port };
        vp = vp::create();
        debug!("vp pointer: {:?}", vp);
        *instance = vp;
        result = vp::connect_universe(vp, dest_domain, dest_port);
        // TODO: Insert event/callback listeners here. Do not listen to CONNECT_UNIVERSE.
    }
    let instance = Instance { vp: vp, attributes: AttribBuffer::new() };
    let mut globals = GLOBALS.lock().unwrap();
    globals.current = vp as usize;
    globals.instances.insert(vp as usize, instance);
    rc(result)
 }