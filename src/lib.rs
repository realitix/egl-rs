#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate  libc;

use libc::c_void;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Rust wrapper
pub fn get_display(native_display: *mut c_void) -> EGLDisplay {
    unsafe { eglGetDisplay(native_display as _) }
}

pub fn initialize(display: EGLDisplay) -> (EGLint, EGLint) {
    let mut major: EGLint = 0;
    let mut minor: EGLint = 0;
    unsafe { eglInitialize(display, &mut major as *mut EGLint, &mut minor as *mut EGLint) };

    (major, minor)
}

pub fn get_configs(display: EGLDisplay) -> Vec<EGLConfig> {
    let mut num_configs: EGLint = 0;
    unsafe { eglGetConfigs(display, ptr::null_mut(), 0, &mut num_configs as *mut EGLint) };
    
    let mut configs = Vec::<EGLConfig>::with_capacity(num_configs as usize);
    unsafe {
        eglGetConfigs(display, (&mut configs[..]).as_mut_ptr(), num_configs, &mut num_configs as *mut EGLint);
        configs.set_len(num_configs as usize);
    };
    configs
}

pub fn get_config_attrib(display: EGLDisplay, config: EGLConfig, attribute: EGLint) -> EGLint {
    let mut val: EGLint = 0;
    unsafe { eglGetConfigAttrib(display, config, attribute, &mut val) };
    val
}

pub fn choose_config(display: EGLDisplay, attrib_list: Vec<EGLint>) -> Vec<EGLConfig> {
    let mut num_configs: EGLint = 0;
    unsafe { eglChooseConfig(display, attrib_list[..].as_ptr(), ptr::null_mut(), 0, &mut num_configs as *mut EGLint) };

    let mut configs = Vec::<EGLConfig>::with_capacity(num_configs as usize);
    unsafe {
        eglChooseConfig(display, attrib_list[..].as_ptr(), (&mut configs[..]).as_mut_ptr(), num_configs, &mut num_configs as *mut EGLint);
        configs.set_len(num_configs as usize);
    }
    configs
}