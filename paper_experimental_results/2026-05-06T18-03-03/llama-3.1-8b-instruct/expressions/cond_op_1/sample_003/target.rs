fn main() {
    let c_vp: *const () = std::ptr::null();
    let vp: *mut () = std::ptr::null_mut();
    let c_ip: *const i32 = std::ptr::null();
    let v_ip: *mut i32 = std::ptr::null_mut();
    let ip: *mut i32 = std::ptr::null_mut();
    let c_cp: *const i8 = std::ptr::null();

    fn f<T>(p: *const T) {}
    fn f1<T>(p: *mut T) {}
    fn f2<T>(p: *mut T) where T: Copy {}
    fn f4<T>(p: *const T) {}
    fn f3<T>(p: *const T) where T: Copy {}

    f(1 as i32? c_vp as *const () : c_ip as *const i32);
    f(1 as i32? c_ip as *const i32 : c_vp as *const ());

    f2(1 as i32? v_ip as *mut i32 : std::ptr::null_mut());
    f2(1 as i32? std::ptr::null_mut() : v_ip as *mut i32);

    f3(1 as i32? c_ip as *const i32 : v_ip as *mut i32);
    f3(1 as i32? v_ip as *mut i32 : c_ip as *const i32);

    f(1 as i32? vp as *mut () : c_cp as *const i8);
    f(1 as i32? c_cp as *const i8 : vp as *mut ());

    f4(1 as i32? ip as *mut i32 : c_ip as *const i32);
    f4(1 as i32? c_ip as *const i32 : ip as *mut i32);

    f1(1 as i32? vp as *mut () : ip as *mut i32);
    f1(1 as i32? ip as *mut i32 : vp as *mut ());
}