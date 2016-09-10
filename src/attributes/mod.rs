use std::os::raw::{c_int, c_char};
use std::ffi::CString;
use std::collections::HashMap;

use raw::{aw, vp};

pub mod mapping;

pub enum AttribValue {
    Int(c_int),
    String(CString),
    Float(f32),
    Data(Vec<u8>)
}

pub struct AttribBuffer {
    pub attribs: HashMap<aw::ATTRIBUTE, AttribValue>
}

impl AttribBuffer {
    pub fn new() -> Self {
        AttribBuffer {
            attribs: HashMap::with_capacity(aw::ATTRIBUTE::MAX_ATTRIBUTE as usize)
        }
    }
    pub fn set<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE, value: T) {
        self.attribs.insert(attribute, value.to_attrib());
    }
    pub fn get<T: Attrib>(&self, attribute: aw::ATTRIBUTE) -> Option<T> {
        self.attribs.get(&attribute).and_then(|a| Attrib::from_attrib(a))
    }
}

pub trait Attrib: Sized {
    fn to_attrib(self) -> AttribValue;
    fn from_attrib(orig: &AttribValue) -> Option<Self>;
    fn default() -> Self;
    
    fn into_req<Other: Attrib>(self) -> Option<Other> {
        Attrib::from_attrib(&self.to_attrib())
    }
}

impl Attrib for c_int {
    fn to_attrib(self) -> AttribValue {
        AttribValue::Int(self)
    }
    fn from_attrib(orig: &AttribValue) -> Option<Self> {
        match *orig {
            AttribValue::Int(val) => Some(val),
            _            => None
        }
    }
    fn default() -> Self { 0 }
}

impl Attrib for CString {
    fn to_attrib(self) -> AttribValue {
        AttribValue::String(self)
    }
    fn from_attrib(orig: &AttribValue) -> Option<Self> {
        match *orig {
            AttribValue::String(ref val) => Some(val.clone()),
            _            => None
        }
    }
    fn default() -> Self { CString::new("").unwrap() }
}

impl Attrib for *mut c_char {
    fn to_attrib(self) -> AttribValue {
        unsafe {
            AttribValue::String(CString::from_raw(self))
        }
    }
    fn from_attrib(orig: &AttribValue) -> Option<Self> {
        match *orig {
            AttribValue::String(ref val) => Some(val.as_ptr() as *mut _),
            _            => None
        }
    }
    fn default() -> Self {
        CString::new("").expect("Really? It's the empty string!").into_raw()
    }
}

impl Attrib for f32 {
    fn to_attrib(self) -> AttribValue {
        AttribValue::Float(self)
    }
    fn from_attrib(orig: &AttribValue) -> Option<Self> {
        match *orig {
            AttribValue::Float(val) => Some(val),
            _            => None
        }
    }
    fn default() -> Self { 0.0 }
}

impl Attrib for Vec<u8> {
    fn to_attrib(self) -> AttribValue {
        AttribValue::Data(self)
    }
    fn from_attrib(orig: &AttribValue) -> Option<Self> {
        match *orig {
            AttribValue::Data(ref val) => Some(val.clone()),
            _            => None
        }
    }
    fn default() -> Self { Vec::new() }
}

