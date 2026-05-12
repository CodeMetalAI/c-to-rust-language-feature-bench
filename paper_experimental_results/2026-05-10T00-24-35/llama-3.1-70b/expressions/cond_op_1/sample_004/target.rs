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
    let c_cp: *const u8 = std::ptr::null();

    f(std::ptr::null());
    f(std::ptr::null());

    f2(std::ptr::null_mut());
    f2(std::ptr::null_mut());

    f3(std::ptr::null());
    f3(std::ptr::null());

    f(std::ptr::null());
    f(std::ptr::null());

    f4(std::ptr::null());
    f4(std::ptr::null());

    f1(std::ptr::null_mut());
    f1(std::ptr::null_mut());
}