fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *mut i32) {}
fn f3(_p: *const i32) {}
fn f4(_p: *const i32) {}

fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const u8 = std::ptr::null();

    f(if 1 != 0 { c_vp } else { c_ip as *const () });
    f(if 1 != 0 { c_ip as *const () } else { c_vp });

    f2(if 1 != 0 { v_ip } else { std::ptr::null_mut() });
    f2(if 1 != 0 { std::ptr::null_mut() } else { v_ip });

    f3(if 1 != 0 { c_ip } else { v_ip as *const i32 });
    f3(if 1 != 0 { v_ip as *const i32 } else { c_ip });

    f(if 1 != 0 { vp as *const () } else { c_cp as *const () });
    f(if 1 != 0 { c_cp as *const () } else { vp as *const () });

    f4(if 1 != 0 { ip as *const i32 } else { c_ip });
    f4(if 1 != 0 { c_ip } else { ip as *const i32 });

    f1(if 1 != 0 { vp } else { ip as *mut () });
    f1(if 1 != 0 { ip as *mut () } else { vp });
}