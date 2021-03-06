use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::os::raw::{c_int, c_void};

use raw::aw;
use raw::vp::{self, VPInstance};
use rc;

use instance::Instance;

use query::SequenceNums;

pub struct Globals {
    pub aw_events: HashMap<aw::EVENT_ATTRIBUTE, extern "C" fn()>,
    pub aw_callbacks: HashMap<aw::CALLBACK, extern "C" fn(rc: c_int)>,
    pub vp_event_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance)+'static>>>,
    pub vp_callback_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance, c_int, c_int)+'static>>>,
    pub current: usize,
    pub instances: HashMap<usize, Instance>,
    pub delayed: Vec<Box<FnMut()+'static>>,
    pub sequence_nums: SequenceNums
}

unsafe impl Send for Globals {}

lazy_static! {
    pub static ref GLOBALS: Mutex<Globals> = {
        Mutex::new(Globals {
            aw_events: HashMap::new(),
            aw_callbacks: HashMap::new(),
            vp_event_closures: HashMap::new(),
            vp_callback_closures: HashMap::new(),
            current: 0,
            instances: HashMap::new(),
            delayed: Vec::new(),
            sequence_nums: SequenceNums::new()
        })
    };
}

impl Globals {
    pub fn current_instance(&self) -> Result<&Instance, c_int> {
        if self.current == 0 { return Err(rc::aw::RC_NO_INSTANCE); }
        self.instances.get(&self.current).ok_or(rc::aw::RC_INVALID_INSTANCE)
    }
    pub fn current_instance_mut(&mut self) -> Result<&mut Instance, c_int> {
        let current = self.current;
        if current == 0 { return Err(rc::aw::RC_NO_INSTANCE); }
        self.instances.get_mut(&current).ok_or(rc::aw::RC_INVALID_INSTANCE)
    }
    pub fn delay<F: FnMut()+'static>(&mut self, f: F) {
        self.delayed.push(Box::new(f));
    }
}

pub fn vp(globals: Option<&Globals>) -> VPInstance {
    let current = match globals {
        Some(globals) => globals.current,
        None => GLOBALS.lock().unwrap().current
    };
    current as *mut c_void
}