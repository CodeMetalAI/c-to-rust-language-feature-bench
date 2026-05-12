use std::ffi::c_void;
use std::os::raw::c_char;

static C_VP: *const c_void = std::ptr::null();
static VP: *mut c_void = std::ptr::null_mut();
static C_IP: *const i32 = std::ptr::null();
static V_IP: *mut i32 = std::ptr::null_mut();
static IP: *mut i32 = std::ptr::null_mut();
static C_CP: *const c_char = std::ptr::null();

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
    f(if true { C_VP } else { C_IP as *const c_void });
    f(if true { C_IP as *const c_void } else { C_VP });

    f2(if true { V_IP } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { V_IP });

    f3(if true { C_IP } else { V_IP as *const i32 });
    f3(if true { V_IP as *const i32 } else { C_IP });

    f(if true { VP as *const c_void } else { C_CP as *const c_void });
    f(if true { C_CP as *const c_void } else { VP as *const c_void });

    f4(if true { IP as *const i32 } else { C_IP });
    f4(if true { C_IP } else { IP as *const i32 });

    f1(if true { VP } else { IP as *mut c_void });
    f1(if true { IP as *mut c_void } else { VP });
}