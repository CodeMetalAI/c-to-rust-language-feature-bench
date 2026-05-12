fn main() {
    let mut x = 17;
    let p: *mut i32 = &mut x;

    let pc: *const i32 = p;
    let pv: *mut i32 = p;
    let pcv: *const i32 = p;

    if pc as *const std::ffi::c_void != p as *const std::ffi::c_void {
        return 1;
    }
    if pv as *const std::ffi::c_void != p as *const std::ffi::c_void {
        return 2;
    }
    if pcv as *const std::ffi::c_void != p as *const std::ffi::c_void {
        return 3;
    }

    if *pc != 17 {
        return 4;
    }
    if *pv != 17 {
        return 5;
    }
    if *pcv != 17 {
        return 6;
    }

    0
}