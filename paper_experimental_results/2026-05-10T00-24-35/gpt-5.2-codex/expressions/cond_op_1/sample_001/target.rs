use std::ffi::c_void;
use std::ptr;

static C_VP: *const c_void = ptr::null();
static VP: *mut c_void = ptr::null_mut();
static C_IP: *const i32 = ptr::null();
static V_IP: *mut i32 = ptr::null_mut();
static IP: *mut i32 = ptr::null_mut();
static C_CP: *const i8 = ptr::null();

fn f(p: *const c_void) {
    let _ = p;
}
fn f1(p: *mut c_void) {
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
    f(if 1 != 0 { C_VP } else { C_IP as *const c_void });
    f(if 1 != 0 { C_IP as *const c_void } else { C_VP });

    f2(if 1 != 0 { V_IP } else { ptr::null_mut() });
    f2(if 1 != 0 { ptr::null_mut() } else { V_IP });

    f3(if 1 != 0 { C_IP } else { V_IP as *const i32 });
    f3(if 1 != 0 { V_IP as *const i32 } else { C_IP });

    f(if 1 != 0 { VP as *const c_void } else { C_CP as *const c_void });
    f(if 1 != 0 { C_CP as *const c_void } else { VP as *const c_void });

    f4(if 1 != 0 { IP as *const i32 } else { C_IP });
    f4(if 1 != 0 { C_IP } else { IP as *const i32 });

    f1(if 1 != 0 { VP } else { IP as *mut c_void });
    f1(if 1 != 0 { IP as *mut c_void } else { VP });
}