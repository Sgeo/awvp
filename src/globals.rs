use std::collections::HashMap;
use std::sync::Mutex;
use std::os::raw::c_int;

use raw::aw;
use raw::vp;

use instance::Instance;

struct Globals {
    aw_events: HashMap<aw::EVENT_ATTRIBUTE, extern "C" fn()>,
    aw_callbacks: HashMap<aw::CALLBACK, extern "C" fn(rc: c_int)>,
    current: usize,
    instances: HashMap<usize, Instance>
}

lazy_static! {
    static ref GLOBALS: Mutex<Globals> = {
        Mutex::new(Globals {
            aw_events: HashMap::new(),
            aw_callbacks: HashMap::new(),
            current: 0,
            instances: HashMap::new()
        })
    };
}