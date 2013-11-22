/* automatically generated by rust-bindgen */

#[feature(globs)]

use std::libc::*;
use std::str::raw;
use std::ptr::to_unsafe_ptr;

pub type khronos_int32_t = int32_t;
pub type khronos_uint32_t = uint32_t;
pub type khronos_int64_t = int64_t;
pub type khronos_uint64_t = uint64_t;
pub type khronos_int8_t = c_schar;
pub type khronos_uint8_t = c_uchar;
pub type khronos_int16_t = c_short;
pub type khronos_uint16_t = c_ushort;
pub type khronos_intptr_t = c_long;
pub type khronos_uintptr_t = c_ulong;
pub type khronos_ssize_t = c_long;
pub type khronos_usize_t = c_ulong;
pub type khronos_float_t = c_float;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_stime_nanoseconds_t = khronos_int64_t;
pub type ANativeWindow = c_void;
pub type egl_native_pixmap_t = c_void;
pub type EGLNativeWindowType = *ANativeWindow;
pub type EGLNativePixmapType = *egl_native_pixmap_t;
pub type EGLNativeDisplayType = *c_void;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeWindowType = EGLNativeWindowType;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = c_uint;
pub type EGLenum = c_uint;
pub type EGLConfig = *c_void;
pub type EGLContext = *c_void;
pub type EGLDisplay = *c_void;
pub type EGLSurface = *c_void;
pub type EGLClientBuffer = *c_void;
pub type __eglMustCastToProperFunctionPointerType = *u8;
pub type status_t = int32_t;

pub static EGL_CONTEXT_CLIENT_VERSION: c_uint = 0x3098;
pub static EGL_NO_CONTEXT: c_uint = 0;
pub static EGL_DEFAULT_DISPLAY: c_uint = 0;
pub static EGL_NONE: c_uint = 0x3038;  // Attrib list terminator
pub static EGL_NO_DISPLAY: c_uint = 0;
pub static EGL_TRUE: c_uint = 1;
pub static EGL_FALSE: c_uint = 0;
pub static EGL_NO_SURFACE: c_uint = 0;
pub static EGL_SURFACE_TYPE: c_uint = 0x3033;
pub static EGL_WINDOW_BIT: c_uint = 0x0004;     // EGL_SURFACE_TYPE mask bits
pub static EGL_RENDERABLE_TYPE: c_uint = 0x3040;
pub static EGL_OPENGL_ES2_BIT: c_uint = 0x0004; // EGL_RENDERABLE_TYPE mask bits
pub static EGL_HEIGHT: c_uint = 0x3056;
pub static EGL_WIDTH: c_uint = 0x3057;

pub static EGL_BUFFER_SIZE: c_uint = 0x3020;
pub static EGL_DEPTH_SIZE: c_uint = 0x3025;


pub fn CreateDisplaySurface() -> EGLNativeWindowType {
    unsafe {
        return android_createDisplaySurface();
    }
}

#[cfg(target_os = "android")]
#[link_args = "-lui"]
#[no_link]
extern {
    pub fn android_createDisplaySurface() -> EGLNativeWindowType;
}

/*
// FIXME 
pub fn SelectConfigForNativeWindow(dpy: EGLDisplay, attrs: &mut EGLint, window: EGLNativeWindowType,
                                    outConfig: &mut EGLConfig) -> status_t {
    unsafe {
        return selectConfigForNativeWindow(dpy, to_unsafe_ptr(attrs), window, to_unsafe_ptr(outConfig));
    }
}

#[cfg(target_os = "android")]
#[link_args = "-lui"]
#[no_link]
extern {
    pub fn selectConfigForNativeWindow(dpy: EGLDisplay, attrs: *EGLint,
                                           window: EGLNativeWindowType,
                                           outConfig: *EGLConfig) -> status_t;
}
*/

#[nolink]
#[link_args = "-lEGL"]
#[cfg(target_os = "android")]
extern {}

pub fn GetError() -> EGLint {
    unsafe {
        return eglGetError();
    }
}

pub fn GetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
    unsafe {
        return eglGetDisplay(display_id);
    }
}

pub fn Initialize(dpy: EGLDisplay, major: &mut EGLint, minor: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglInitialize(dpy, to_unsafe_ptr(major), to_unsafe_ptr(minor));
    }
}

pub fn Terminate(dpy: EGLDisplay) -> EGLBoolean {
    unsafe {
        return eglTerminate(dpy);
    }
}

pub fn QueryString(dpy: EGLDisplay, name: EGLint) -> ~str {
    unsafe {
        return raw::from_c_str(eglQueryString(dpy, name));
    }
}

pub fn GetConfigs(dpy: EGLDisplay, configs: &mut EGLConfig, config_size: EGLint, num_config: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglGetConfigs(dpy, to_unsafe_ptr(configs), config_size, to_unsafe_ptr(num_config));
    }
}

pub fn ChooseConfig(dpy: EGLDisplay, attrib_list: *EGLint, configs: &mut EGLConfig, config_size: EGLint, num_config: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglChooseConfig(dpy, attrib_list, to_unsafe_ptr(configs), config_size, to_unsafe_ptr(num_config));
    }
}

pub fn GetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglGetConfigAttrib(dpy, config, attribute, to_unsafe_ptr(value));
    }
}

pub fn CreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *EGLint) -> EGLSurface {
    unsafe {
        return eglCreateWindowSurface(dpy, config, win, attrib_list);
    }
}

pub fn CreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePbufferSurface(dpy, config, to_unsafe_ptr(attrib_list));
    }
}

pub fn CreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePixmapSurface(dpy, config, pixmap, to_unsafe_ptr(attrib_list));
    }
}

pub fn DestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe {
        return eglDestroySurface(dpy, surface);
    }
}

pub fn QuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglQuerySurface(dpy, surface, attribute, to_unsafe_ptr(value));
    }
}

pub fn BindAPI(api: EGLenum) -> EGLBoolean {
    unsafe {
        return eglBindAPI(api);
    }
}

pub fn QueryAPI() -> EGLenum {
    unsafe {
        return eglQueryAPI();
    }
}

pub fn WaitClient() -> EGLBoolean {
    unsafe {
        return eglWaitClient();
    }
}

pub fn ReleaseThread() -> EGLBoolean {
    unsafe {
        return eglReleaseThread();
    }
}

pub fn CreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePbufferFromClientBuffer(dpy, buftype, buffer, config, to_unsafe_ptr(attrib_list));
    }
}

pub fn SurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean {
    unsafe {
        return eglSurfaceAttrib(dpy, surface, attribute, value);
    }
}

pub fn BindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean {
    unsafe {
        return eglBindTexImage(dpy, surface, buffer);
    }
}

pub fn ReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean {
    unsafe {
        return eglReleaseTexImage(dpy, surface, buffer);
    }
}

pub fn SwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean {
    unsafe {
        return eglSwapInterval(dpy, interval);
    }
}

pub fn CreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *EGLint) -> EGLContext {
    unsafe {
        return eglCreateContext(dpy, config, share_context, attrib_list);
    }
}

pub fn DestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean {
    unsafe {
        return eglDestroyContext(dpy, ctx);
    }
}

pub fn MakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean {
    unsafe {
        return eglMakeCurrent(dpy, draw, read, ctx);
    }
}

pub fn GetCurrentContext() -> EGLContext {
    unsafe {
        return eglGetCurrentContext();
    }
}

pub fn GetCurrentSurface(readdraw: EGLint) -> EGLSurface {
    unsafe {
        return eglGetCurrentSurface(readdraw);
    }
}

pub fn GetCurrentDisplay() -> EGLDisplay {
    unsafe {
        return eglGetCurrentDisplay();
    }
}

pub fn QueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglQueryContext(dpy, ctx, attribute, to_unsafe_ptr(value));
    }
}

pub fn WaitGL() -> EGLBoolean {
    unsafe {
        return eglWaitGL();
    }
}

pub fn WaitNative(engine: EGLint) -> EGLBoolean {
    unsafe {
        return eglWaitNative(engine);
    }
}

pub fn SwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe {
        return eglSwapBuffers(dpy, surface);
    }
}

pub fn CopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean {
    unsafe {
        return eglCopyBuffers(dpy, surface, target);
    }
}

#[nolink]
extern {
    fn eglGetError() -> EGLint;
    fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
    fn eglInitialize(dpy: EGLDisplay, major: *EGLint, minor: *EGLint) -> EGLBoolean;
    fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *c_schar;
    fn eglGetConfigs(dpy: EGLDisplay, configs: *EGLConfig, config_size: EGLint, num_config: *EGLint) -> EGLBoolean;
    fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *EGLint, configs: *EGLConfig, config_size: EGLint, num_config: *EGLint) -> EGLBoolean;
    fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *EGLint) -> EGLSurface;
    fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: *EGLint) -> EGLSurface;
    fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: *EGLint) -> EGLSurface;
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    fn eglQueryAPI() -> EGLenum;
    fn eglWaitClient() -> EGLBoolean;
    fn eglReleaseThread() -> EGLBoolean;
    fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: *EGLint) -> EGLSurface;
    fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean;
    fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
    fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *EGLint) -> EGLContext;
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean;
    fn eglGetCurrentContext() -> EGLContext;
    fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
    fn eglGetCurrentDisplay() -> EGLDisplay;
    fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglWaitGL() -> EGLBoolean;
    fn eglWaitNative(engine: EGLint) -> EGLBoolean;
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean;
    fn eglGetProcAddress(procname: *c_schar) -> __eglMustCastToProperFunctionPointerType;
}
