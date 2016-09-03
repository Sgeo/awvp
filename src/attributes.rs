use std::os::raw::{c_int};
use std::ffi::CString;
use std::collections::HashMap;

use raw::{aw, vp};

pub enum AttribValue {
    Int(c_int),
    String(CString),
    Bool(c_int),
    Float(f32),
    Data(Vec<u8>)
}

pub struct AttribBuffer {
    pub attribs: HashMap<c_int, AttribValue>
}

impl AttribBuffer {
    pub fn new() -> Self {
        AttribBuffer {
            attribs: HashMap::with_capacity(aw::MAX_ATTRIBUTE as usize)
        }
    }
}