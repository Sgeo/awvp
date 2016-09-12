use std::sync::{Arc, Mutex};

use raw::vp::{self, VPInstance};
use globals::GLOBALS;
use instance::Instance;

use std::os::raw::c_int;

macro_rules! generate_callback {
    ($this_callback_name:expr, $current_callback_name:expr, $current_instance:expr, $activate:expr) => {{
        if $this_callback_name == $current_callback_name {
            debug!("generate_callback({:?}, {:?}, ..., {:?})", $this_callback_name, $current_callback_name, $activate);
            extern "C" fn callback(instance: VPInstance, arg1: c_int, arg2: c_int) {
                debug!("Inside native vp callback");
                let globals = GLOBALS.lock().unwrap();
                let maybe_closure = globals.instances.get(&(instance as usize)).and_then(|i| i.vp_callback_closures.get(&$this_callback_name)).map(|callback| callback.clone());
                match maybe_closure {
                    Some(closure) => {
                        drop(globals);
                        closure(instance, arg1, arg2);
                    },
                    None => { debug!("Attempted to call closure not present!") }
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
                let globals = GLOBALS.lock().unwrap();
                let maybe_closure = globals.instances.get(&(instance as usize)).and_then(|i| i.vp_event_closures.get(&$this_event_name)).map(|handler| handler.clone());
                match maybe_closure {
                    Some(closure) => {
                        drop(globals);
                        closure(instance)
                    },
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

pub fn callback_closure_set(instance: &mut Instance, callback_name: vp::callback_t, closure: Option<Arc<Box<Fn(VPInstance, c_int, c_int)+'static>>>) {
    debug!("callback_closure_set({:?}, {:?}, {:?})", instance.vp, callback_name, closure.as_ref().map(|_| Some(())));
    if let Some(closure) = closure {
        instance.vp_callback_closures.insert(callback_name, closure);
        activate_callback(instance.vp, callback_name, true);
    } else {
        activate_callback(instance.vp, callback_name, false);
        instance.vp_callback_closures.remove(&callback_name);
    }
}

pub fn event_closure_set(instance: &mut Instance, event_name: vp::event_t, closure: Option<Arc<Box<Fn(VPInstance)+'static>>>) {
    if let Some(closure) = closure {
        instance.vp_event_closures.insert(event_name, closure);
        activate_event(instance.vp, event_name, true);
    } else {
        activate_event(instance.vp, event_name, false);
        instance.vp_event_closures.remove(&event_name);
    }
}

pub fn callback_closure_set_all<F: Fn(VPInstance, c_int, c_int)+'static>(callback_name: vp::callback_t, closure: Option<F>) {
    debug!("callback_closure_set_all({:?}, {:?})", callback_name, closure.as_ref().map(|_| Some(())));
    let closure = closure.map(|c| Arc::new(Box::new(c) as Box<Fn(VPInstance, c_int, c_int)+'static>));
    let mut globals = GLOBALS.lock().unwrap();
    for instance in globals.instances.values_mut() {
        let closure_clone = closure.as_ref().map(|c| c.clone());
        callback_closure_set(instance, callback_name, closure_clone);
    }
    match closure {
        Some(closure) => {
            globals.vp_callback_closures.insert(callback_name, closure);
        },
        None          => {
            globals.vp_callback_closures.remove(&callback_name);
        }
    }
}

pub fn event_closure_set_all<F: Fn(VPInstance)+'static>(event_name: vp::callback_t, closure: Option<F>) {
    let closure = closure.map(|c| Arc::new(Box::new(c) as Box<Fn(VPInstance)+'static>));
    let mut globals = GLOBALS.lock().unwrap();
    for instance in globals.instances.values_mut() {
        let closure_clone = closure.as_ref().map(|c| c.clone());
        event_closure_set(instance, event_name, closure_clone);
    }
    match closure {
        Some(closure) => {
            globals.vp_event_closures.insert(event_name, closure);
        },
        None          => {
            globals.vp_event_closures.remove(&event_name);
        }
    }
}