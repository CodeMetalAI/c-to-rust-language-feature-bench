fn f(p: *const ()) {}
fn f1(p: *mut ()) {}
fn f2(p: *mut i32) {}
fn f3(p: *const i32) {}
fn f4(p: *const i32) {}

fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const u8 = std::ptr::null();

    f(if true { c_vp } else { c_ip as *const () });
    f(if true { c_ip as *const () } else { c_vp });

    f2(if true { v_ip } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { v_ip });

    f3(if true { c_ip as *const i32 } else { v_ip as *const i32 });
    f3(if true { v_ip as *const i32 } else { c_ip as *const i32 });

    f(if true { vp } else { c_cp as *const () });
    f(if true { c_cp as *const () } else { vp });

    f4(if true { ip as *const i32 } else { c_ip });
    f4(if true { c_ip } else { ip as *const i32 });

    f1(if true { vp } else { ip as *mut () });
    f1(if true { ip as *mut () } else { vp });
}