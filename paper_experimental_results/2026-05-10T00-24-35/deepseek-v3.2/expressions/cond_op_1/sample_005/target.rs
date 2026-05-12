use std::ptr;

static mut c_vp: *const std::ffi::c_void = ptr::null();
static mut vp: *mut std::ffi::c_void = ptr::null_mut();
static mut c_ip: *const i32 = ptr::null();
static mut v_ip: *mut i32 = ptr::null_mut();
static mut ip: *mut i32 = ptr::null_mut();
static mut c_cp: *const std::os::raw::c_char = ptr::null();

fn f(p: *const std::ffi::c_void) {
    let _ = p;
}

fn f1(p: *mut std::ffi::c_void) {
    let _ = p;
}

fn f2(p: *mut i32) {
    let _ = p;
}

fn f4(p: *const i32) {
    let _ = p;
}

fn f3(p: *const i32) {
    let _ = p;
}

fn main() {
    unsafe {
        f(if true { c_vp } else { c_ip as *const std::ffi::c_void });
        f(if true { c_ip as *const std::ffi::c_void } else { c_vp });

        f2(if true { v_ip } else { ptr::null_mut() });
        f2(if true { ptr::null_mut() } else { v_ip });

        f3(if true { c_ip } else { v_ip as *const i32 });
        f3(if true { v_ip as *const i32 } else { c_ip });

        f(if true { vp as *const std::ffi::c_void } else { c_cp as *const std::ffi::c_void });
        f(if true { c_cp as *const std::ffi::c_void } else { vp as *const std::ffi::c_void });

        f4(if true { ip as *const i32 } else { c_ip });
        f4(if true { c_ip } else { ip as *const i32 });

        f1(if true { vp } else { ip as *mut std::ffi::c_void });
        f1(if true { ip as *mut std::ffi::c_void } else { vp });
    }
}