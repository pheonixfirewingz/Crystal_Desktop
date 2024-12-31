use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use libloading::Library;

pub mod framebuffer;
pub mod texture;

static GL_LOADED: AtomicBool = AtomicBool::new(false);
pub fn init_gl() {
    // If GL is already loaded, return immediately
    if GL_LOADED.load(Ordering::Acquire) {
        return;
    }
    let egl = unsafe { Library::new("libEGL.so.1") }.expect("Failed to load libEGL.so.1");
    let get_proc_address: libloading::Symbol<
        unsafe extern "C" fn(*const i8) -> *mut std::ffi::c_void,
    > = unsafe {
        egl.get(b"eglGetProcAddress")
            .expect("eglGetProcAddress failed to be retrieved")
    };

    gl::load_with(|s| {
        let name = CString::new(s).unwrap();
        unsafe { get_proc_address(name.as_ptr()) as *const _ }
    });
    // Keep egl loaded by deliberately leaking the ptr
    std::mem::forget(egl);
    GL_LOADED.store(true, Ordering::Release);
}