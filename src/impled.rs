use std::os::raw::{c_int, c_char, c_void, c_uint};
use std::ffi::CStr;

use globals::{GLOBALS, vp};

use raw::{aw, vp};

use instance::Instance;

use attributes::mapping::InstanceExt;

use rc::rc;

#[no_mangle]
pub extern fn aw_init() -> c_int {
    extern crate flexi_logger;
    let mut config = flexi_logger::LogConfig::new();
    config.log_to_file = true;
    flexi_logger::init(config, None).expect("Unable to initialize logger");
    
    debug!("aw_init();");
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
    let instance = Instance::new(vp);
    let mut globals = GLOBALS.lock().unwrap();
    globals.current = vp as usize;
    globals.instances.insert(vp as usize, instance);
    rc(result)
 }
 
#[no_mangle]
pub extern fn aw_int(a: aw::ATTRIBUTE) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().get(a).unwrap();
    debug!("aw_int({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_int_set(a: aw::ATTRIBUTE, value: c_int) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    globals.current_instance_mut().set(a, value);
    debug!("aw_int_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_bool(a: aw::ATTRIBUTE) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().get(a).unwrap();
    debug!("aw_bool({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_bool_set(a: aw::ATTRIBUTE, value: c_int) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    globals.current_instance_mut().set(a, value);
    debug!("aw_bool_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_float(a: aw::ATTRIBUTE) -> f32 {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().get(a).unwrap();
    debug!("aw_float({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_float_set(a: aw::ATTRIBUTE, value: f32) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    globals.current_instance_mut().set(a, value);
    debug!("aw_float_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_string(a: aw::ATTRIBUTE) -> *mut c_char {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().get(a).unwrap();
    debug!("aw_string({:?}) = {:?}", a, unsafe { CStr::from_ptr(result as *const _) });
    result
}

#[no_mangle]
pub extern fn aw_string_set(a: aw::ATTRIBUTE, value: *mut c_char) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    globals.current_instance_mut().set(a, value);
    debug!("aw_string_set({:?}, {:?});", a, unsafe { CStr::from_ptr(value as *const _) });
    0
}

#[no_mangle]
pub extern fn aw_data(a: aw::ATTRIBUTE, lenptr: *mut c_uint) -> *mut c_char {
    let mut globals = GLOBALS.lock().unwrap();
    let result: (*mut c_void, c_uint) = globals.current_instance_mut().get(a).unwrap();
    unsafe {
        *lenptr = result.1 as c_uint
    }
    debug!("aw_string({:?}) = ...", a);
    result.0 as *mut c_char
}

#[no_mangle]
pub extern fn aw_data_set(a: aw::ATTRIBUTE, value: *mut c_char, len: c_uint) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    globals.current_instance_mut().set(a, (value as *mut c_void, len));
    debug!("aw_data_set({:?}, ...;", a);
    0
}

#[no_mangle]
pub extern fn aw_wait(milliseconds: c_int) -> c_int {
    let instance = vp(None);
    let mut result = 0;
    debug!("aw_wait({:?});", milliseconds);
    if milliseconds < 0 {
        loop {
            unsafe {
                result = vp::wait(instance, 0);
                if result != 0 { return rc(result); }
            }
        }
    } else {
        use std::time::{Instant, Duration};
        let start = Instant::now();
        let duration = Duration::from_millis(milliseconds as u64);
        while start.elapsed() < duration {
            unsafe {
                result = vp::wait(instance, milliseconds);
                if result != 0 { return rc(result); }
            }
        }
        return result;
    }
}