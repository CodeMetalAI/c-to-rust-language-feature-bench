use std::ffi::c_void;
use std::ptr;

const c_vp: *const c_void = ptr::null();
const vp: *mut c_void = ptr::null_mut();
const c_ip: *const i32 = ptr::null();
const v_ip: *mut i32 = ptr::null_mut();
const ip: *mut i32 = ptr::null_mut();
const c_cp: *const u8 = ptr::null();

fn f(_p: *const c_void) {}
fn f1(_p: *mut c_void) {}
fn f2(_p: *mut i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    f(if true { c_vp } else { c_ip as *const c_void });
    f(if true { c_ip as *const c_void } else { c_vp });
    f2(if true { v_ip } else { ptr::null_mut() });
    f2(if true { ptr::null_mut() } else { v_ip });
    f3(if true { c_ip } else { v_ip as *const i32 });
    f3(if true { v_ip as *const i32 } else { c_ip });
    f(if true { vp as *const c_void } else { c_cp as *const c_void });
    f(if true { c_cp as *const c_void } else { vp as *const c_void });
    f4(if true { ip as *const i32 } else { c_ip });
    f4(if true { c_ip } else { ip as *const i32 });
    f1(if true { vp } else { ip as *mut c_void });
    f1(if true { ip as *mut c_void } else { vp });
}