use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_char};
use std::slice;

use raw::{aw, vp};
use attributes::{AttribValue, Attrib};
use instance::Instance;

fn vp_string(instance: &mut Instance, vp_attribute: vp::string_attribute_t) -> CString {
    unsafe {
        CStr::from_ptr(vp::string(instance.vp, vp_attribute) as *const c_char).to_owned()
    }
}

fn cint(val: c_int) -> c_int { val }

fn coord_aw_to_vp(aw: c_int) -> f32 {
    (aw as f32) / 1000.0
}

fn coord_vp_to_aw(vp: f32) -> c_int {
    (vp * 1000.0) as c_int
}


unsafe fn debug_vp_string(vp: vp::VPInstance, attribute: vp::string_attribute_t) -> *mut c_char {
    let vpstring = vp::string(vp, attribute);
    if vpstring.is_null() {
        debug!("vp_string({:?}, {:?}) is null", vp, attribute);
    } else {
        let copy = CStr::from_ptr(vpstring as *const c_char).to_owned();
        debug!("vp_string({:?}, {:?}) = {:?}", vp, attribute, copy);
    }
    vpstring
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
            aw::ATTRIBUTE::WORLD_NAME => vp_string(self, vp::WORLD_NAME).into_req(),
            aw::ATTRIBUTE::WORLD_BUILD_NUMBER => 69.into_req(), // Needed for Xelagot to whisper, 69 = latest 3.6
            aw::ATTRIBUTE::CHAT_MESSAGE => vp_string(self, vp::CHAT_MESSAGE).into_req(),
            aw::ATTRIBUTE::AVATAR_NAME => vp_string(self, vp::AVATAR_NAME).into_req(),
            aw::ATTRIBUTE::AVATAR_SESSION => unsafe { vp::int(self.vp, vp::AVATAR_SESSION) }.into_req(),
            aw::ATTRIBUTE::CHAT_SESSION => unsafe { vp::int(self.vp, vp::AVATAR_SESSION) }.into_req(),
            aw::ATTRIBUTE::WORLD_SPEAK_CAPABILITY => 1.into_req(),
            aw::ATTRIBUTE::WORLD_ALLOW_TOURIST_WHISPER => 1.into_req(),
            aw::ATTRIBUTE::WORLD_ALLOW_CITIZEN_WHISPER => 1.into_req(),
            aw::ATTRIBUTE::WORLD_SPEAK_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::WORLD_BUILD_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::WORLD_ENTER_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::WORLD_SPECIAL_OBJECTS_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::WORLD_BOTS_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::WORLD_SPECIAL_COMMANDS_RIGHT => CString::new("*").ok().and_then(|cstr| cstr.into_req()),
            aw::ATTRIBUTE::MY_X => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::MY_X) }).into_req(),
            aw::ATTRIBUTE::MY_Z => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::MY_Z) }).into_req(),
            aw::ATTRIBUTE::MY_Y => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::MY_Y) }).into_req(),
            aw::ATTRIBUTE::AVATAR_X => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::AVATAR_X) }).into_req(),
            aw::ATTRIBUTE::AVATAR_Z => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::AVATAR_Z) }).into_req(),
            aw::ATTRIBUTE::AVATAR_Y => coord_vp_to_aw(unsafe { vp::float(self.vp, vp::AVATAR_Y) }).into_req(),
            aw::ATTRIBUTE::AVATAR_PRIVILEGE => unsafe { vp::int(self.vp, vp::USER_ID) }.into_req(),
            aw::ATTRIBUTE::AVATAR_CITIZEN => unsafe {
                let name = vp_string(self, vp::AVATAR_NAME);
                if name.as_bytes().first() == Some(&b'[') {
                    0
                } else {
                    vp::int(self.vp, vp::USER_ID) 
                }
            }.into_req(),
            _ => self.attributes.get(attribute)
        }
    }
    
    fn set<T: Attrib>(&mut self, attribute: aw::ATTRIBUTE, value: T) {
        match attribute {
            aw::ATTRIBUTE::WORLD_NAME => unsafe { vp::string_set(self.vp, vp::WORLD_NAME, value.into_req().expect("Wrong type for attribute!")); },
            aw::ATTRIBUTE::CHAT_MESSAGE => unsafe { vp::string_set(self.vp, vp::CHAT_MESSAGE, value.into_req().expect("Wrong type for attribute!")); },
            aw::ATTRIBUTE::MY_X => unsafe { vp::float_set(self.vp, vp::MY_X, coord_aw_to_vp(value.into_req().expect("Wrong type for attribute!"))); },
            aw::ATTRIBUTE::MY_Z => unsafe { vp::float_set(self.vp, vp::MY_Z, coord_aw_to_vp(value.into_req().expect("Wrong type for attribute!"))); },
            aw::ATTRIBUTE::MY_Y => unsafe { vp::float_set(self.vp, vp::MY_Y, coord_aw_to_vp(value.into_req().expect("Wrong type for attribute!"))); },
            _ => self.attributes.set(attribute, value)
        }
    }
}