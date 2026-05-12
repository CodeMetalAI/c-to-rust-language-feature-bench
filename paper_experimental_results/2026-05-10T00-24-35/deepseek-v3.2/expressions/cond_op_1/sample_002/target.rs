use std::ptr;

static C_VP: *const std::ffi::c_void = ptr::null();
static mut VP: *mut std::ffi::c_void = ptr::null_mut();
static C_IP: *const i32 = ptr::null();
static mut V_IP: *mut i32 = ptr::null_mut();
static mut IP: *mut i32 = ptr::null_mut();
static C_CP: *const std::ffi::c_char = ptr::null();

fn f(_p: *const std::ffi::c_void) {}
fn f1(_p: *mut std::ffi::c_void) {}
fn f2(_p: *mut i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    unsafe {
        f(if true { C_VP } else { C_IP as *const std::ffi::c_void });
        f(if true { C_IP as *const std::ffi::c_void } else { C_VP });

        f2(if true { V_IP } else { ptr::null_mut() });
        f2(if true { ptr::null_mut() } else { V_IP });

        f3(if true { C_IP } else { V_IP as *const i32 });
        f3(if true { V_IP as *const i32 } else { C_IP });

        f(if true { VP as *const std::ffi::c_void } else { C_CP as *const std::ffi::c_void });
        f(if true { C_CP as *const std::ffi::c_void } else { VP as *const std::ffi::c_void });

        f4(if true { IP as *const i32 } else { C_IP });
        f4(if true { C_IP } else { IP as *const i32 });

        f1(if true { VP } else { IP as *mut std::ffi::c_void });
        f1(if true { IP as *mut std::ffi::c_void } else { VP });
    }
}