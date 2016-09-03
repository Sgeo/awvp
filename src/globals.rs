use std::collections::HashMap;
use std::sync::Mutex;
use std::os::raw::c_int;

use raw::aw;
use raw::vp;

use instance::Instance;

pub struct Globals {
    pub aw_events: HashMap<aw::EVENT_ATTRIBUTE, extern "C" fn()>,
    pub aw_callbacks: HashMap<aw::CALLBACK, extern "C" fn(rc: c_int)>,
    pub current: usize,
    pub instances: HashMap<usize, Instance>
}

lazy_static! {
    pub static ref GLOBALS: Mutex<Globals> = {
        Mutex::new(Globals {
            aw_events: HashMap::new(),
            aw_callbacks: HashMap::new(),
            current: 0,
            instances: HashMap::new()
        })
    };
}

impl Globals {
    pub fn current_instance(&self) -> &Instance {
        self.instances.get(&self.current).expect("Unable to find current instance!")
    }
}