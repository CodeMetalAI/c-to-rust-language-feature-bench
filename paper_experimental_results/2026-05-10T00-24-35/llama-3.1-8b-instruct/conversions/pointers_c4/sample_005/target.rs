fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const std::ffi::c_void = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: fn(i32) -> i32 = None;
    let fp1: fn(i32) -> i32 = v0 as fn(i32) -> i32;
    let fp2: fn(i32) -> i32 = (fp1 as fn(i32) -> i32);

    if std::ptr::eq(p0, p1) {
        panic!("p0 != p1");
    }
    if !std::ptr::eq(p0, std::ptr::null()) {
        panic!("p0 != std::ptr::null()");
    }

    if !std::ptr::eq(v0, std::ptr::null()) {
        panic!("v0 != std::ptr::null()");
    }
    if !std::ptr::eq(v0, p0 as *const std::ffi::c_void) {
        panic!("v0 != p0 as *const std::ffi::c_void");
    }

    if !std::ptr::eq(fp0, fp1) {
        panic!("fp0 != fp1");
    }
    if !std::ptr::eq(fp1, fp2) {
        panic!("fp1 != fp2");
    }
    if !std::ptr::eq(fp0, std::ptr::null()) {
        panic!("fp0 != std::ptr::null()");
    }

    if !std::ptr::eq(p0, fp0 as *const i32) {
        panic!("p0 != fp0 as *const i32");
    }

    assert!(true);
}