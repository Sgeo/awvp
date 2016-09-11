

use raw::{aw, vp};

use std::os::raw::c_int;

use std::collections::HashMap;
use std::sync::Arc;

use attributes::AttribBuffer;

pub struct Instance {
    pub vp: vp::VPInstance,
    pub vp_event_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance)+'static>>>,
    pub vp_callback_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance, c_int, c_int)+'static>>>,
    pub attributes: AttribBuffer
}

impl Instance {
    pub fn new(vp: vp::VPInstance) -> Self {
        Instance {
            vp: vp,
            attributes: AttribBuffer::new(),
            vp_event_closures: HashMap::new(),
            vp_callback_closures: HashMap::new()
        }
    }
}

unsafe impl Send for Instance {}

