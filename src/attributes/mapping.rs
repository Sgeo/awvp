use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_char};
use std::slice;

use raw::{aw, vp};
use attributes::{AttribValue, Attrib};
use instance::Instance;

fn vp_string(instance: &mut Instance, aw_attribute: aw::ATTRIBUTE, vp_attribute: vp::string_attribute_t) -> *mut c_char {
    let s = unsafe {
        CStr::from_ptr(vp::string(instance.vp, vp_attribute))
    }.to_owned();
    instance.attributes.set(aw_attribute, s);
    instance.attributes.get(aw_attribute).unwrap()
}

fn vp_data(vp: vp::VPInstance, attribute: vp::data_attribute_t) -> Vec<u8> {
    let mut length: c_int = 0;
    unsafe {
        let ptr = vp::data(vp, attribute, &mut length) as *const u8;
        let slice = slice::from_raw_parts(ptr, length as usize);
        slice.to_owned()
    }
}

pub trait InstanceExt {
    fn get<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE) -> Option<T>;
    fn set<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE, value: T);
}

impl InstanceExt for Instance {
    fn get<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE) -> Option<T> {
        match attribute {
            aw::ATTRIBUTE::CITIZEN_NUMBER => unsafe { vp::int(self.vp, vp::USER_ID) }.into_req(),
            aw::ATTRIBUTE::WORLD_NAME => vp_string(self, aw::ATTRIBUTE::WORLD_NAME, vp::WORLD_NAME).into_req(),
            _ => self.attributes.get(attribute)
        }
    }
    
    fn set<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE, value: T) {
        match attribute {
            _ => self.attributes.set(attribute, value)
        }
    }
}