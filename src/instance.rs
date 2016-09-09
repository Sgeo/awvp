

use raw::{aw, vp};

use std::os::raw::c_int;

use std::collections::HashMap;

use attributes::AttribBuffer;

pub struct Instance {
    pub vp: vp::VPInstance,
    pub vp_event_closures: HashMap<vp::event_t, Box<FnMut(vp::VPInstance)>>,
    pub vp_callback_closures: HashMap<vp::event_t, Box<FnMut(vp::VPInstance, c_int, c_int)>>,
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

