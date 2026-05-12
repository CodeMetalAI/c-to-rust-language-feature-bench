use std::ptr;

fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *mut i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    let c_vp: *const () = ptr::null();
    let vp: *mut () = ptr::null_mut();
    let c_ip: *const i32 = ptr::null();
    let v_ip: *mut i32 = ptr::null_mut();
    let ip: *mut i32 = ptr::null_mut();
    let c_cp: *const i8 = ptr::null();

    f(if true { c_vp } else { c_ip as *const () });
    f(if true { c_ip as *const () } else { c_vp });

    f2(if true { v_ip } else { ptr::null_mut() });
    f2(if true { ptr::null_mut() } else { v_ip });

    f3(if true { c_ip } else { v_ip as *const i32 });
    f3(if true { v_ip as *const i32 } else { c_ip });

    f(if true { vp as *const () } else { c_cp as *const () });
    f(if true { c_cp as *const () } else { vp as *const () });

    f4(if true { ip as *const i32 } else { c_ip });
    f4(if true { c_ip } else { ip as *const i32 });

    f1(if true { vp } else { ip as *mut () });
    f1(if true { ip as *mut () } else { vp });

    std::process::exit(0);
}