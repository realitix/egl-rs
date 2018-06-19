#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use libc::c_void;
use std::ptr;

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Types
pub type Display = sys::EGLDisplay;
pub type Int = sys::EGLint;
pub type NativeWindowType = sys::NativeWindowType;
pub type Config = sys::EGLConfig;
pub type Surface = sys::EGLSurface;

// Constant
pub static SURFACE_TYPE: Int = sys::EGL_SURFACE_TYPE as Int;
pub static WINDOW_BIT: Int = sys::EGL_WINDOW_BIT as Int;
pub static RENDERABLE_TYPE: Int = sys::EGL_RENDERABLE_TYPE as Int;
pub static OPENGL_ES2_BIT: Int = sys::EGL_OPENGL_ES2_BIT as Int;
pub static RED_SIZE: Int = sys::EGL_RED_SIZE as Int;
pub static GREEN_SIZE: Int = sys::EGL_GREEN_SIZE as Int;
pub static BLUE_SIZE: Int = sys::EGL_BLUE_SIZE as Int;
pub static NONE: Int = sys::EGL_NONE as Int;

// Rust wrapper
pub fn get_display(native_display: *mut c_void) -> Display {
    unsafe { sys::eglGetDisplay(native_display as _) }
}

pub fn initialize(display: Display) -> (Int, Int) {
    let mut major: Int = 0;
    let mut minor: Int = 0;
    unsafe { sys::eglInitialize(display, &mut major as *mut Int, &mut minor as *mut Int) };

    (major, minor)
}

pub fn get_configs(display: Display) -> Vec<Config> {
    let mut num_configs: Int = 0;
    unsafe { sys::eglGetConfigs(display, ptr::null_mut(), 0, &mut num_configs as *mut Int) };

    let mut configs = Vec::<Config>::with_capacity(num_configs as usize);
    unsafe {
        sys::eglGetConfigs(
            display,
            (&mut configs[..]).as_mut_ptr(),
            num_configs,
            &mut num_configs as *mut Int,
        );
        configs.set_len(num_configs as usize);
    };
    configs
}

pub fn get_config_attrib(display: Display, config: Config, attribute: Int) -> Int {
    let mut val: Int = 0;
    unsafe { sys::eglGetConfigAttrib(display, config, attribute, &mut val) };
    val
}

pub fn choose_config(display: Display, attrib_list: Vec<Int>) -> Vec<Config> {
    let mut num_configs: Int = 0;
    unsafe {
        sys::eglChooseConfig(
            display,
            attrib_list[..].as_ptr(),
            ptr::null_mut(),
            0,
            &mut num_configs as *mut Int,
        )
    };

    let mut configs = Vec::<Config>::with_capacity(num_configs as usize);
    unsafe {
        sys::eglChooseConfig(
            display,
            attrib_list[..].as_ptr(),
            (&mut configs[..]).as_mut_ptr(),
            num_configs,
            &mut num_configs as *mut Int,
        );
        configs.set_len(num_configs as usize);
    }
    configs
}

pub fn create_window_surface(
    display: Display,
    config: Config,
    native_window: NativeWindowType,
    attrib_list: Option<Vec<Int>>,
) -> Surface {
    let attr = match attrib_list {
        Some(x) => x,
        None => vec![NONE],
    };

    unsafe { sys::eglCreateWindowSurface(display, config, native_window, attr[..].as_ptr()) }
}
