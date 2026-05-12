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

    f(1 as usize != 0 ? c_vp as *const () : c_ip as *const ());
    f(1 as usize != 0 ? c_ip as *const () : c_vp as *const ());

    f2(1 as usize != 0 ? v_ip : std::ptr::null_mut());
    f2(1 as usize != 0 ? std::ptr::null_mut() : v_ip);

    f3(1 as usize != 0 ? c_ip as *const i32 : v_ip as *const i32);
    f3(1 as usize != 0 ? v_ip as *const i32 : c_ip as *const i32);

    f(1 as usize != 0 ? vp as *const () : c_cp as *const ());
    f(1 as usize != 0 ? c_cp as *const () : vp as *const ());

    f4(1 as usize != 0 ? ip as *const i32 : c_ip as *const i32);
    f4(1 as usize != 0 ? c_ip as *const i32 : ip as *const i32);

    f1(1 as usize != 0 ? vp as *mut () : ip as *mut ());
    f1(1 as usize != 0 ? ip as *mut () : vp as *mut ());
}