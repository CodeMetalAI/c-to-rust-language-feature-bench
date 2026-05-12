fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    unsafe {
        f(c_vp as *const _ as *const ());
        f(vp as *const _ as *const ());
        f2(v_ip);
        f2(0 as *mut _);
        f3(c_ip as *const _);
        f3(v_ip);
        f(vp as *const _ as *const ());
        f(c_cp as *const _ as *const ());
        f4(ip as *const _);
        f4(c_ip as *const _);
        f1(vp as *mut _);
        f1(ip as *mut _);
    }

    std::process::exit(0);
}

fn f<T>(_p: *const T) {}
fn f1<T>(_p: *mut T) {}
fn f2<T>(_p: *mut T) {}
fn f3<T>(_p: *const T) {}
fn f4<T>(_p: *const T) {}