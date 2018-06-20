#include <EGL/egl.h>
#include <EGL/eglext.h>

// Bindgen doesn't support macro defined constant for now
// https://github.com/rust-lang-nursery/rust-bindgen/issues/316
#define EGL_NO_CONTEXT  0
#define EGL_NO_DISPLAY  0
#define EGL_NO_SURFACE  0