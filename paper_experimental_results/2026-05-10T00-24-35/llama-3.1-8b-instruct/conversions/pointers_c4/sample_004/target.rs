fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const std::ffi::c_void = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<&dyn Fn(i32) -> i32> = None;
    let fp1: Option<&dyn Fn(i32) -> i32> = None;
    let fp2: Option<&dyn Fn(i32) -> i32> = Some(f);

    if p0 != p1 {
        panic!("p0 != p1");
    }
    if p0 != std::ptr::null() {
        panic!("p0 != std::ptr::null()");
    }

    if v0 != std::ptr::null() {
        panic!("v0 != std::ptr::null()");
    }
    if v0 != p0 as *const std::ffi::c_void {
        panic!("v0 != p0 as *const std::ffi::c_void");
    }

    if fp0 != fp1 {
        panic!("fp0 != fp1");
    }
    if fp1 != fp2 {
        panic!("fp1 != fp2");
    }
    if fp0 != None {
        panic!("fp0 != None");
    }

    if p0 != fp0 as *const i32 {
        panic!("p0 != fp0 as *const i32");
    }

    assert_eq!(0, 0);
}