fn f(_p: *const std::ffi::c_void) {}
fn f1(_p: *mut std::ffi::c_void) {}
fn f2(_p: *mut i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    let c_vp: *const std::ffi::c_void = std::ptr::null();
    let vp: *mut std::ffi::c_void = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const std::ffi::c_char = std::ptr::null();

    f(if true { c_vp } else { c_ip });
    f(if true { c_ip } else { c_vp });

    f2(if true { v_ip } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { v_ip });

    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    f(if true { vp } else { c_cp });
    f(if true { c_cp } else { vp });

    f4(if true { ip } else { c_ip });
    f4(if true { c_ip } else { ip });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });
}