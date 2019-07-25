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
pub type Context = sys::EGLContext;

// Constant
pub const SURFACE_TYPE: Int = sys::EGL_SURFACE_TYPE as Int;
pub const WINDOW_BIT: Int = sys::EGL_WINDOW_BIT as Int;
pub const RENDERABLE_TYPE: Int = sys::EGL_RENDERABLE_TYPE as Int;
pub const OPENGL_ES2_BIT: Int = sys::EGL_OPENGL_ES2_BIT as Int;
pub const RED_SIZE: Int = sys::EGL_RED_SIZE as Int;
pub const GREEN_SIZE: Int = sys::EGL_GREEN_SIZE as Int;
pub const BLUE_SIZE: Int = sys::EGL_BLUE_SIZE as Int;
pub const NONE: Int = sys::EGL_NONE as Int;
pub const NO_CONTEXT: Context = sys::EGL_NO_CONTEXT as Context;
pub const CONTEXT_CLIENT_VERSION: Int = sys::EGL_CONTEXT_CLIENT_VERSION as Int;
pub const NO_SURFACE: Surface = sys::EGL_NO_SURFACE as Surface;

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
    attrib_list: Vec<Int>,
) -> Surface {
    unsafe { sys::eglCreateWindowSurface(display, config, native_window, attrib_list[..].as_ptr()) }
}

pub fn create_context(
    display: Display,
    config: Config,
    share_context: Context,
    attrib_list: Vec<Int>,
) -> Context {
    unsafe { sys::eglCreateContext(display, config, share_context, attrib_list[..].as_ptr()) }
}

pub fn make_current(display: Display, draw: Surface, read: Surface, context: Context) {
    unsafe { sys::eglMakeCurrent(display, draw, read, context) };
}

pub fn swap_buffers(display: Display, surface: Surface) {
    unsafe { sys::eglSwapBuffers(display, surface) };
}
