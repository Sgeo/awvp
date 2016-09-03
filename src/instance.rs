

use raw::{aw, vp};

use attributes::AttribBuffer;

pub struct Instance {
    pub vp: vp::VPInstance,
    pub attributes: AttribBuffer
}

unsafe impl Send for Instance {}

