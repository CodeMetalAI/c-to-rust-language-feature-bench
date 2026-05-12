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
    let c_cp: *const i8 = std::ptr::null();

    f(1 as usize != 0 ? c_vp : c_ip);
    f(1 as usize == 0 ? c_ip : c_vp);

    f2(1 as usize != 0 ? v_ip : std::ptr::null_mut());
    f2(1 as usize == 0 ? std::ptr::null_mut() : v_ip);

    f3(1 as usize != 0 ? c_ip : v_ip as *const i32);
    f3(1 as usize == 0 ? v_ip as *const i32 : c_ip);

    f(1 as usize != 0 ? vp : c_cp as *const ());
    f(1 as usize == 0 ? c_cp as *const () : vp);

    f4(1 as usize != 0 ? ip : c_ip);
    f4(1 as usize == 0 ? c_ip : ip);

    f1(1 as usize != 0 ? vp : ip);
    f1(1 as usize == 0 ? ip : vp);
}