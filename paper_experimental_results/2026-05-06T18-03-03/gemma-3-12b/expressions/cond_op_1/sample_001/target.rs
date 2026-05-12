fn main() {
    let c_vp: *const std::ffi::c_void = std::ptr::null();
    let vp: *mut std::ffi::c_void = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *volatile i32 = std::ptr::null();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    fn f(_p: *const std::ffi::c_void) {}
    fn f1(_p: *mut std::ffi::c_void) {}
    fn f2(_p: *volatile i32) {}
    fn f4(_p: *const i32) {}
    fn f3(_p: *const volatile i32) {}

    f(if 1 { c_vp } else { c_ip });
    f(if 1 { c_ip } else { c_vp });

    f2(if 1 { v_ip } else { std::ptr::null() });
    f2(if 1 { std::ptr::null() } else { v_ip });

    f3(if 1 { c_ip } else { v_ip });
    f3(if 1 { v_ip } else { c_ip });

    f(if 1 { vp } else { c_cp });
    f(if 1 { c_cp } else { vp });

    f4(if 1 { ip } else { c_ip });
    f4(if 1 { c_ip } else { ip });

    f1(if 1 { vp } else { ip });
    f1(if 1 { ip } else { vp });
}