fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const u8 = std::ptr::null();

    f(c_vp);
    f(c_ip as *const ());

    f2(v_ip);
    f2(std::ptr::null_mut());

    f3(c_ip as *const i32);
    f3(v_ip as *const i32);

    f(vp);
    f(c_cp as *const ());

    f4(ip as *const i32);
    f4(c_ip);

    f1(vp);
    f1(ip as *mut ());
}

fn f(p: *const ()) {
    let _ = p;
}

fn f1(p: *mut ()) {
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