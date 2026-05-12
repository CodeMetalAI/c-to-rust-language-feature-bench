fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const u8 = std::ptr::null();

    f(c_vp, 1);
    f(c_ip, 0);

    f2(v_ip, 1);
    f2(std::ptr::null_mut(), 0);

    f3(c_ip, 1);
    f3(v_ip, 0);

    f(vp, 1);
    f(c_cp, 0);

    f4(c_ip, 1);
    f4(ip, 0);

    f1(vp, 1);
    f1(ip, 0);
}

fn f<T>(_p: *const T, _b: bool) {}
fn f1<T>(_p: *mut T, _b: bool) {}
fn f2<T>(_p: *mut T, _b: bool) {}
fn f3<T>(_p: *const T, _b: bool) {}
fn f4<T>(_p: *const T, _b: bool) {}