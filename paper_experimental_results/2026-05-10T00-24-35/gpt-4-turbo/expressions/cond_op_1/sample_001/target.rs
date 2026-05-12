fn f(p: *const u8) {}
fn f1(p: *mut u8) {}
fn f2(p: *mut u8) {}
fn f3(p: *const u8) {}
fn f4(p: *const u8) {}

fn main() {
    let c_vp: *const u8 = std::ptr::null();
    let vp: *mut u8 = std::ptr::null_mut();
    let c_ip: *const u8 = std::ptr::null();
    let v_ip: *mut u8 = std::ptr::null_mut();
    let ip: *mut u8 = std::ptr::null_mut();
    let c_cp: *const u8 = std::ptr::null();

    f(if true { c_vp } else { c_ip });
    f(if true { c_ip } else { c_vp });

    f2(if true { v_ip } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { v_ip });

    f3(if true { c_ip as *const u8 } else { v_ip as *const u8 });
    f3(if true { v_ip as *const u8 } else { c_ip as *const u8 });

    f(if true { vp as *const u8 } else { c_cp });
    f(if true { c_cp } else { vp as *const u8 });

    f4(if true { ip as *const u8 } else { c_ip });
    f4(if true { c_ip } else { ip as *const u8 });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });
}