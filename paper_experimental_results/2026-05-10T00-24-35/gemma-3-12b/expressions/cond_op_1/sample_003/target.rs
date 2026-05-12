fn main() {
    let c_vp: *const std::ffi::c_void = std::ptr::null();
    let vp: *mut std::ffi::c_void = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *volatile i32 = std::ptr::null();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    fn f(p: *const std::ffi::c_void) {
        // (void)p; // No-op in C, do nothing in Rust
    }
    fn f1(p: *mut std::ffi::c_void) {
        // (void)p; // No-op in C, do nothing in Rust
    }
    fn f2(p: *volatile i32) {
        // (void)p; // No-op in C, do nothing in Rust
    }
    fn f4(p: *const i32) {
        // (void)p; // No-op in C, do nothing in Rust
    }
    fn f3(p: *const volatile i32) {
        // (void)p; // No-op in C, do nothing in Rust
    }


    f(if 1 == 1 { c_vp } else { c_ip });
    f(if 1 == 1 { c_ip } else { c_vp });

    f2(if 1 == 1 { v_ip } else { std::ptr::null_volatile() });
    f2(if 1 == 1 { std::ptr::null() } else { v_ip });

    f3(if 1 == 1 { c_ip } else { v_ip });
    f3(if 1 == 1 { v_ip } else { c_ip });

    f(if 1 == 1 { vp } else { c_cp });
    f(if 1 == 1 { c_cp } else { vp });

    f4(if 1 == 1 { ip } else { c_ip });
    f4(if 1 == 1 { c_ip } else { ip });

    f1(if 1 == 1 { vp } else { ip });
    f1(if 1 == 1 { ip } else { vp });
}