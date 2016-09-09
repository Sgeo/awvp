use std::sync::Mutex;

use raw::vp::{self, VPInstance};
use globals::GLOBALS;

use std::os::raw::c_int;

macro_rules! generate_callback {
    ($this_callback_name:expr, $current_callback_name:expr, $current_instance:expr, $activate:expr) => {{
        if $this_callback_name == $current_callback_name {
            extern "C" fn callback(instance: VPInstance, arg1: c_int, arg2: c_int) {
                let mut globals = GLOBALS.lock().unwrap();
                let maybe_closure = globals.instances.get_mut(&(instance as usize)).and_then(|i| i.vp_callback_closures.get_mut(&$this_callback_name));
                match maybe_closure {
                    Some(closure) => closure(instance, arg1, arg2),
                    None => ()
                }
            }
            unsafe {
                if $activate {
                    vp::callback_set($current_instance, $current_callback_name, Some(callback));
                } else {
                    vp::callback_set($current_instance, $current_callback_name, None);
                }
            }
            return;
        }
    }}
}

macro_rules! generate_event {
    ($this_event_name:expr, $current_event_name:expr, $current_instance:expr, $activate:expr) => {{
        if $this_event_name == $current_event_name {
            extern "C" fn event_handler(instance: VPInstance) {
                let mut globals = GLOBALS.lock().unwrap();
                let maybe_closure = globals.instances.get_mut(&(instance as usize)).and_then(|i| i.vp_event_closures.get_mut(&$this_event_name));
                match maybe_closure {
                    Some(closure) => closure(instance),
                    None => ()
                }
            }
            unsafe {
                if $activate {
                    vp::event_set($current_instance, $current_event_name, Some(event_handler));
                } else {
                    vp::event_set($current_instance, $current_event_name, None);
                }
            }
            return;
        }
    }}
}

pub fn activate_callback(vp: VPInstance, callback_name: vp::callback_t, activate: bool) {
    generate_callback!(vp::CALLBACK_OBJECT_ADD, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_OBJECT_CHANGE, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_OBJECT_DELETE, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_GET_FRIENDS, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_FRIEND_ADD, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_FRIEND_DELETE, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_TERRAIN_QUERY, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_TERRAIN_NODE_SET, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_OBJECT_GET, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_OBJECT_LOAD, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_LOGIN, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_ENTER, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_JOIN, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_CONNECT_UNIVERSE, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_WORLD_PERMISSION_USER_SET, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_WORLD_PERMISSION_SESSION_SET, callback_name, vp, activate);
    generate_callback!(vp::CALLBACK_WORLD_SETTING_SET, callback_name, vp, activate);
}

pub fn activate_event(vp: VPInstance, event_name: vp::event_t, activate:bool) {
    generate_event!(vp::EVENT_CHAT, event_name, vp, activate);
    generate_event!(vp::EVENT_AVATAR_ADD, event_name, vp, activate);
    generate_event!(vp::EVENT_AVATAR_CHANGE, event_name, vp, activate);
    generate_event!(vp::EVENT_AVATAR_DELETE, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT_CHANGE, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT_DELETE, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT_CLICK, event_name, vp, activate);
    generate_event!(vp::EVENT_WORLD_LIST, event_name, vp, activate);
    generate_event!(vp::EVENT_WORLD_SETTING, event_name, vp, activate);
    generate_event!(vp::EVENT_WORLD_SETTINGS_CHANGED, event_name, vp, activate);
    generate_event!(vp::EVENT_FRIEND, event_name, vp, activate);
    generate_event!(vp::EVENT_WORLD_DISCONNECT, event_name, vp, activate);
    generate_event!(vp::EVENT_UNIVERSE_DISCONNECT, event_name, vp, activate);
    generate_event!(vp::EVENT_USER_ATTRIBUTES, event_name, vp, activate);
    generate_event!(vp::EVENT_CELL_END, event_name, vp, activate);
    generate_event!(vp::EVENT_TERRAIN_NODE, event_name, vp, activate);
    generate_event!(vp::EVENT_AVATAR_CLICK, event_name, vp, activate);
    generate_event!(vp::EVENT_TELEPORT, event_name, vp, activate);
    generate_event!(vp::EVENT_URL, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT_BUMP_BEGIN, event_name, vp, activate);
    generate_event!(vp::EVENT_OBJECT_BUMP_END, event_name, vp, activate);
    generate_event!(vp::EVENT_TERRAIN_NODE_CHANGED, event_name, vp, activate);
    generate_event!(vp::EVENT_JOIN, event_name, vp, activate);
}