

use raw::{aw, vp};

use std::os::raw::c_int;
use std::ffi::CString;

use std::collections::HashMap;
use std::sync::Arc;

use attributes::{AttribBuffer, AttribValue};

pub struct Instance {
    pub vp: vp::VPInstance,
    pub vp_event_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance)+'static>>>,
    pub vp_callback_closures: HashMap<vp::event_t, Arc<Box<Fn(vp::VPInstance, c_int, c_int)+'static>>>,
    pub attributes: AttribBuffer,
    pub overrides: HashMap<aw::ATTRIBUTE, AttribValue>,
    pub citname_to_citnum: HashMap<CString, c_int>
}

impl Instance {
    pub fn new(vp: vp::VPInstance) -> Self {
        Instance {
            vp: vp,
            attributes: AttribBuffer::new(),
            overrides: HashMap::new(),
            vp_event_closures: HashMap::new(),
            vp_callback_closures: HashMap::new(),
            citname_to_citnum: HashMap::new()
        }
    }
}

unsafe impl Send for Instance {}

