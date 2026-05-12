fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    unsafe {
        f(c_vp);
        f(c_ip);

        f2(v_ip);
        f2(std::ptr::null_mut());

        f3(c_ip);
        f3(v_ip);

        f(vp);
        f(c_cp);

        f4(ip);
        f4(c_ip);

        f1(vp);
        f1(ip);
    }
}

fn f(p: *const ()) {
    std::mem::forget(p);
}

fn f1(p: *mut ()) {
    std::mem::forget(p);
}

fn f2(p: *mut i32) {
    std::mem::forget(p);
}

fn f3(p: *const i32) {
    std::mem::forget(p);
}

fn f4(p: *const i32) {
    std::mem::forget(p);
}