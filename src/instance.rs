

use raw::{aw, vp};

use attributes::AttribBuffer;

pub struct Instance {
    pub attributes: AttribBuffer
}

unsafe impl Send for Instance {}

