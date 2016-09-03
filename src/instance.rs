use raw::{aw, vp};

use attributes::AttribBuffer;

pub struct Instance {
    pub vp_instance: vp::VPInstance,
    pub attributes: AttribBuffer
}

unsafe impl Send for Instance {}