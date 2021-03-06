use std::os::raw::{c_int, c_char, c_void, c_uint};
use std::ffi::{CStr, CString};

use globals::{GLOBALS, vp};

use raw::{aw, vp};

use instance::Instance;

use attributes::mapping::InstanceExt;

use rc::rc;

#[cfg(debug_assertions)]
fn init_logging() {
    extern crate flexi_logger;
    extern crate log_panics;
    use std::panic;
    let mut config = flexi_logger::LogConfig::new();
    config.log_to_file = true;
    flexi_logger::init(config, None).expect("Unable to initialize logger");
    
    log_panics::init();
}

#[cfg(not(debug_assertions))]
fn init_logging() {
}


#[no_mangle]
pub extern fn aw_init() -> c_int {

    init_logging();
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
    let instances: Vec<VPInstance>;
    let mut delays: Option<Vec<Box<FnMut()>>>;
    {
        let mut globals = GLOBALS.lock().unwrap();
        instances = globals.instances.keys().map(|iref| *iref as VPInstance).collect();
        let num_of_delays = globals.delayed.len();
        if num_of_delays > 0 {
            delays = Some(::std::mem::replace(&mut globals.delayed, Vec::new()));
        } else {
            delays = None
        }
    }
    for mut delay in delays.iter_mut().flat_map(|delay_vec| delay_vec) {
        delay();
    }
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
        aw::EVENT_ATTRIBUTE::EVENT_AVATAR_ADD => event_closure_set_all(vp::EVENT_AVATAR_ADD, closure.map(|closure| move |vpinstance| {
            {
                let mut globals = GLOBALS.lock().unwrap();
                globals.current = vpinstance as usize;
                let _ = globals.current_instance_mut().map(|instance| {
                    let username: CString = instance.get(aw::ATTRIBUTE::AVATAR_NAME).expect("No valid username in EVENT_AVATAR_ADD");
                    let citnum: c_int = instance.get(aw::ATTRIBUTE::AVATAR_CITIZEN).expect("No valid citizen in EVENT_AVATAR_ADD");
                    debug!("Adding to citnamenum cache: {:?}", (&username, &citnum));
                    if citnum != 0 {
                        instance.citname_to_citnum.insert(username, citnum);
                    }
                });
            };
            closure(vpinstance);
        })),
        aw::EVENT_ATTRIBUTE::EVENT_AVATAR_CHANGE => event_closure_set_all(vp::EVENT_AVATAR_CHANGE, closure),
        aw::EVENT_ATTRIBUTE::EVENT_AVATAR_DELETE => event_closure_set_all(vp::EVENT_AVATAR_DELETE, closure),
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

#[no_mangle]
pub extern fn aw_whisper(session: c_int, message: *const c_char) -> c_int {
    rc(unsafe{
        let mut globals = GLOBALS.lock().unwrap();
        let instance = try_rc!(globals.current_instance_mut());
        let vp = instance.vp;
        let name: CString = try_rc!(instance.get(aw::ATTRIBUTE::LOGIN_NAME).ok_or(::rc::aw::RC_NAME_TOO_SHORT));
        let mut botty_name_vec = Vec::with_capacity(name.as_bytes().len() + 2);
        botty_name_vec.push(b'[');
        botty_name_vec.append(&mut name.into_bytes());
        botty_name_vec.push(b']');
        let botty_name = CString::new(botty_name_vec).expect("Unable to create new CString for name");
        vp::console_message(vp, session, botty_name.as_ptr(), message, vp::TEXT_EFFECT_ITALIC, 0, 0, 255)
    })
}

#[no_mangle]
pub extern fn aw_citizen_attributes_by_name(name: *const c_char) -> c_int {
    debug!("aw_citizen_attributes_by_name(...);");
    let mut globals = GLOBALS.lock().unwrap();
    let current = globals.current;
    let wanted_citname = unsafe {
        CStr::from_ptr(name).to_owned()
    };
    debug!("wanted_citname: {:?}", &wanted_citname);
    let maybe_citnum;
    {
        let instance = try_rc!(globals.current_instance_mut());
        maybe_citnum = instance.citname_to_citnum.get(&wanted_citname).map(|citnum| *citnum);
    }
    debug!("Made it to the check");
    if let Some(citnum) = maybe_citnum {
        debug!("Found a by_name result cached!");
        globals.delay(move || {
            debug!("Inside the delayed callback");
            let maybe_callback;
            {
                let mut globals = GLOBALS.lock().unwrap();
                globals.current = current;
                {
                    let instance = globals.current_instance_mut().expect("No current instance, aw_citizen_attributes_by_name() called without active instance?");
                    instance.set_override(aw::ATTRIBUTE::CITIZEN_NAME, Some(wanted_citname.clone()));
                    instance.set_override(aw::ATTRIBUTE::CITIZEN_NUMBER, Some(citnum));
                }
                maybe_callback = globals.aw_callbacks.get(&aw::CALLBACK::CALLBACK_CITIZEN_ATTRIBUTES).map(|callback| *callback);
            }
            if let Some(callback) = maybe_callback {
                debug!("About to call AW's callback for aw_citizen_attributes_by_name");
                callback(0);
            }
            let mut globals = GLOBALS.lock().unwrap();
            let instance = globals.current_instance_mut().expect("No current instance, aw_citizen_attributes_by_name() called without active instance?");
            instance.set_override::<CString>(aw::ATTRIBUTE::CITIZEN_NAME, None);
            instance.set_override::<c_int>(aw::ATTRIBUTE::CITIZEN_NUMBER, None);
        });
    }
    0
}

#[no_mangle]
pub extern fn aw_sector_from_cell(cell: c_int) -> c_int {
    use query;
    query::sector_from_cell(cell)
}

#[no_mangle]
pub extern fn aw_query_5x5(x_sector: c_int, z_sector: c_int, sequence: *mut [c_int; 5]) -> c_int {
    use std::slice;
    use query;
    let sequence: [[c_int; 5]; 5] = unsafe {
        let sequence = slice::from_raw_parts(sequence as *const _, 5);
        [sequence[0], sequence[1], sequence[2], sequence[3], sequence[4]]
    };
    debug!("aw_query_5x5({:?}, {:?}, {:?});", x_sector, z_sector, sequence);
    let instance = vp(None);
    for (cell_x, cell_z) in query::cell_coords_from_sector_coords(x_sector, z_sector) { // TODO: Whole zone, not just sector
        unsafe {
            vp::query_cell(instance, cell_x, cell_z);
        }
    }
    0
}