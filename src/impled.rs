use std::os::raw::{c_int, c_char, c_void, c_uint};
use std::ffi::{CStr, CString};

use globals::{GLOBALS, vp};

use raw::{aw, vp};

use instance::Instance;

use attributes::mapping::InstanceExt;

use rc::rc;



#[no_mangle]
pub extern fn aw_init() -> c_int {
    extern crate flexi_logger;
    extern crate log_panics;
    use std::panic;
    let mut config = flexi_logger::LogConfig::new();
    config.log_to_file = true;
    flexi_logger::init(config, None).expect("Unable to initialize logger");
    
    log_panics::init();
    
    
    /*panic::set_hook(Box::new(move |panic_info| {
        error!("PANIC!");
        error!("Panic Payload [str]: {:?}", panic_info.payload().downcast_ref::<&'static str>());
        error!("Panic Payload [String]: {:?}", panic_info.payload().downcast_ref::<String>());
        error!("Panic Location: File: {:?}, Line: {:?}", panic_info.location().unwrap().file(), panic_info.location().unwrap().line());
        error!("Backtrace: {:?}", backtrace);
    }));*/
    
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
        debug!("aw_create({:?}, {:?}) = instance: {:?}, rc: {:?}", dest_domain, dest_port, vp, rc(result));
    }
    let mut instance = Instance::new(vp);
    let mut globals = GLOBALS.lock().unwrap();
    
    use ec::{callback_closure_set, event_closure_set};
    
    for (callback_name, closure) in &globals.vp_callback_closures {
        debug!("Adding callback from globals to specific instance");
        callback_closure_set(&mut instance, *callback_name, Some(closure.clone()));
    }
    for (event_name, closure) in &globals.vp_event_closures {
        event_closure_set(&mut instance, *event_name, Some(closure.clone()));
    }
    
    globals.current = vp as usize;
    globals.instances.insert(vp as usize, instance);
    
    rc(result)
 }
 
#[no_mangle]
pub extern fn aw_int(a: aw::ATTRIBUTE) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().ok().and_then(|instance| instance.get(a)).unwrap_or(0);
    debug!("aw_int({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_int_set(a: aw::ATTRIBUTE, value: c_int) -> c_int {
    debug!("aw_int_set");
    let mut globals = GLOBALS.lock().unwrap();
    try_rc!(globals.current_instance_mut()).set(a, value);
    debug!("aw_int_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_bool(a: aw::ATTRIBUTE) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().ok().and_then(|instance| instance.get(a)).unwrap_or(0);
    debug!("aw_bool({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_bool_set(a: aw::ATTRIBUTE, value: c_int) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    try_rc!(globals.current_instance_mut()).set(a, value);
    debug!("aw_bool_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_float(a: aw::ATTRIBUTE) -> f32 {
    let mut globals = GLOBALS.lock().unwrap();
    let result = globals.current_instance_mut().ok().and_then(|instance| instance.get(a)).unwrap_or(0.0);
    debug!("aw_float({:?}) = {:?}", a, result);
    result
}

#[no_mangle]
pub extern fn aw_float_set(a: aw::ATTRIBUTE, value: f32) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    try_rc!(globals.current_instance_mut()).set(a, value);
    debug!("aw_float_set({:?}, {:?});", a, value);
    0
}

#[no_mangle]
pub extern fn aw_string(a: aw::ATTRIBUTE) -> *mut c_char {
    use std::sync::Mutex;
    lazy_static! {
        static ref buffer: Mutex<Option<CString>> = Mutex::new(None);
    }
    let mut globals = GLOBALS.lock().unwrap();
    let mut bufferguard = buffer.lock().unwrap();
    *bufferguard = globals.current_instance_mut().ok().and_then(|instance| instance.get(a));
    debug!("aw_string({:?}) = {:?}", a, &*bufferguard);
    unsafe {
        bufferguard.as_ref().map(|cstring| cstring.as_ptr() as *mut c_char).unwrap_or(::std::ptr::null_mut())
    }
}

#[no_mangle]
pub extern fn aw_string_set(a: aw::ATTRIBUTE, value: *mut c_char) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    try_rc!(globals.current_instance_mut()).set(a, value);
    debug!("aw_string_set({:?}, {:?});", a, unsafe { CStr::from_ptr(value as *const _) });
    0
}

#[no_mangle]
pub extern fn aw_data(a: aw::ATTRIBUTE, lenptr: *mut c_uint) -> *mut c_char {
    let mut globals = GLOBALS.lock().unwrap();
    let result: (*mut c_void, c_uint) = globals.current_instance_mut().ok().and_then(|instance| instance.get(a)).unwrap_or((0x1 as *mut c_void, 0));
    unsafe {
        *lenptr = result.1 as c_uint
    }
    debug!("aw_string({:?}) = ...", a);
    result.0 as *mut c_char
}

#[no_mangle]
pub extern fn aw_data_set(a: aw::ATTRIBUTE, value: *mut c_char, len: c_uint) -> c_int {
    let mut globals = GLOBALS.lock().unwrap();
    try_rc!(globals.current_instance_mut()).set(a, (value as *mut c_void, len));
    debug!("aw_data_set({:?}, ...;", a);
    0
}

#[no_mangle]
pub extern fn aw_wait(milliseconds: c_int) -> c_int {
    use raw::vp::VPInstance;
    let instances: Vec<VPInstance> = GLOBALS.lock().unwrap().instances.keys().map(|iref| *iref as VPInstance).collect();
    let mut result = 0;
    if milliseconds < 0 {
        loop {
            unsafe {
                for instance in &instances {
                    result = vp::wait(*instance, 0);
                    if result != 0 { return rc(result); }
                }
            }
        }
    } else {
        use std::time::{Instant, Duration};
        let start = Instant::now();
        let duration = Duration::from_millis(milliseconds as u64);
        while (milliseconds == 0) || (start.elapsed() <= duration) {
            unsafe {
                for instance in &instances {
                    result = vp::wait(*instance, 0);
                    if result != 0 { return rc(result); }
                }
            }
            if milliseconds == 0 { break; }
        }
        return result;
    }
}

#[no_mangle]
pub extern fn aw_instance() -> *mut c_void {
    let result = GLOBALS.lock().unwrap().current as *mut c_void;
    result
}

#[no_mangle]
pub extern fn aw_instance_set(instance: *mut c_void) -> c_int {
    GLOBALS.lock().unwrap().current = instance as usize;
    0
}

#[no_mangle]
pub extern fn aw_login() -> c_int {
    debug!("aw_login");
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    
    let citnum: c_int;
    let password: CString;
    let botname: CString;
    let mut citname: Option<CString> = None;
    {
        let mut globals = GLOBALS.lock().unwrap();
        citnum = try_rc!(globals.current_instance_mut()).get(aw::ATTRIBUTE::LOGIN_OWNER).unwrap();
        password = try_rc!(globals.current_instance_mut()).get(aw::ATTRIBUTE::LOGIN_PRIVILEGE_PASSWORD).unwrap();
        botname = try_rc!(globals.current_instance_mut()).get(aw::ATTRIBUTE::LOGIN_NAME).unwrap();
    }
    let prefix = format!("{}=", citnum);
    let citizens_file = BufReader::new(File::open("citizens.txt").expect("Unable to find citizens.txt"));
    for line in citizens_file.lines() {
        let line = line.expect("???");
        if line.starts_with(&prefix) {
            citname = Some(CString::new(line.split('=').nth(1).expect("No =foo?")).expect("Unable to create CString"));
            break
        }
    }
    debug!("aw_login() [AW citnum: {:?}, VP citname: {:?}, Botname: {:?}]", citnum, citname.as_ref().expect("Unable to find citname"), &botname);
    let result = unsafe {
        rc(vp::login(vp(None), citname.expect("Unable to find citname").as_ptr(), password.as_ptr(), botname.as_ptr()))
    };
    //::ec::call_callback_closure(GLOBALS.lock().unwrap(), vp::CALLBACK_LOGIN, result)
    result
}

#[no_mangle]
pub extern fn aw_callback(callback_name: aw::CALLBACK) -> Option<extern "C" fn(rc: c_int)> {
    let globals = GLOBALS.lock().unwrap();
    globals.aw_callbacks.get(&callback_name).map(|callback| *callback)
}

#[no_mangle]
pub extern fn aw_event(event_name: aw::EVENT_ATTRIBUTE) -> Option<extern "C" fn()> {
    let globals = GLOBALS.lock().unwrap();
    globals.aw_events.get(&event_name).map(|handler| *handler)
}

#[no_mangle]
pub extern fn aw_callback_set(callback_name: aw::CALLBACK, callback: Option<extern "C" fn(rc: c_int)>) -> c_int {
    use ec::{callback_closure_set_all, event_closure_set_all};
    
    debug!("aw_callback_set({:?}, ...);", callback_name);
    let mut globals = GLOBALS.lock().unwrap();
    match callback {
        None => { globals.aw_callbacks.remove(&callback_name); () },
        Some(callback) => { globals.aw_callbacks.insert(callback_name, callback); () }
    };
    drop(globals);
    let closure = move |instance: vp::VPInstance, rc_: c_int, unused: c_int| {
        debug!("Inside a VP callback closure!");
        let aw_callback;
        {
            let mut globals = GLOBALS.lock().unwrap();
            aw_callback = *globals.aw_callbacks.get(&callback_name).expect("Unable to find aw_callback!");
            globals.current = instance as usize;
        }
        aw_callback(rc(rc_));
    };
    let closure = callback.map(|_| closure);
    match callback_name {
        aw::CALLBACK::CALLBACK_LOGIN => callback_closure_set_all(vp::CALLBACK_LOGIN, closure),
        aw::CALLBACK::CALLBACK_ENTER => callback_closure_set_all(vp::CALLBACK_ENTER, closure),
        aw::CALLBACK::CALLBACK_CITIZEN_ATTRIBUTES => event_closure_set_all(vp::EVENT_USER_ATTRIBUTES, closure.map(|closure| move |instance| closure(instance, 0, 0))),
        _                            => { debug!("No mapping for callback!"); ()}
    }
    0
}

#[no_mangle]
pub extern fn aw_event_set(event_name: aw::EVENT_ATTRIBUTE, handler: Option<extern "C" fn()>) -> c_int {
    use ec::{callback_closure_set_all, event_closure_set_all};
    
    debug!("aw_event_set({:?}, ...);", event_name);
    let mut globals = GLOBALS.lock().unwrap();
    match handler {
        None => { globals.aw_events.remove(&event_name); () },
        Some(handler) => { globals.aw_events.insert(event_name, handler); () }
    };
    drop(globals);
    let closure = move |instance: vp::VPInstance| {
        debug!("Inside a VP event closure!");
        let aw_event;
        {
            let mut globals = GLOBALS.lock().unwrap();
            aw_event = *globals.aw_events.get(&event_name).expect("Unable to find aw_event!");
            globals.current = instance as usize;
        }
        aw_event();
    };
    let closure = handler.map(|_| closure);
    match event_name {
        aw::EVENT_ATTRIBUTE::EVENT_CHAT => event_closure_set_all(vp::EVENT_CHAT, closure),
        aw::EVENT_ATTRIBUTE::EVENT_WORLD_ATTRIBUTES => event_closure_set_all(vp::EVENT_WORLD_SETTINGS_CHANGED, closure),
        _                            => { debug!("No mapping for event!"); ()}
    }
    0
}

#[no_mangle] pub extern fn aw_enter(world: *const c_char) -> c_int {
    debug!("aw_enter({:?});", unsafe { CStr::from_ptr(world) });
    let result = rc(unsafe {
        vp::enter(vp(None), world)
    });
    //::ec::call_callback_closure(GLOBALS.lock().unwrap(), vp::CALLBACK_LOGIN, result)
    result
}

#[no_mangle]
pub extern fn aw_state_change() -> c_int {
    rc(unsafe{
        vp::state_change(vp(None))
    })
}

#[no_mangle]
pub extern fn aw_say(msg: *const c_char) -> c_int {
    rc(unsafe{
        vp::say(vp(None), msg)
    })
}

#[no_mangle]
pub extern fn aw_citizen_attributes_by_number(citizen: c_int) -> c_int {
    rc(unsafe{
        vp::user_attributes_by_id(vp(None), citizen)
    })
}