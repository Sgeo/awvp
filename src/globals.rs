use std::collections::HashMap;
use std::sync::Mutex;
use std::os::raw::c_int;

use raw::aw;
use raw::vp;

struct Globals {
    aw_events: HashMap<aw::EVENT_ATTRIBUTE, extern "C" fn()>,
    aw_callbacks: HashMap<aw::CALLBACK, extern "C" fn(rc: c_int)>
}

lazy_static! {
    static ref GLOBALS: Mutex<Globals> = {
        Mutex::new(Globals {
            aw_events: HashMap::new(),
            aw_callbacks: HashMap::new()
        })
    };
}