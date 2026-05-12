fn main() {
    let c_vp: *const i32 = std::ptr::null();
    let vp: *mut i32 = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    unsafe {
        f(c_vp, 1);
        f(c_ip, 1);

        f2(v_ip, 1);
        f2(std::ptr::null_mut(), 1);

        f3(c_ip, 1);
        f3(v_ip, 1);

        f(vp, 1);
        f(c_cp, 1);

        f4(ip, 1);
        f4(c_ip, 1);

        f1(vp, 1);
        f1(ip, 1);
    }
}

fn f<T>(p: *const T, _b: bool) {
    let _p = p;
}

fn f1<T>(p: *mut T, _b: bool) {
    let _p = p;
}

fn f2<T>(p: *mut T, _b: bool) {
    let _p = p;
}

fn f3<T>(p: *const T, _b: bool) {
    let _p = p;
}

fn f4<T>(p: *const T, _b: bool) {
    let _p = p;
}