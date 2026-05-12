fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const std::ffi::c_void = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = Some(f);
    let fp2: Option<fn(i32) -> i32> = v0 as Option<fn(i32) -> i32>;

    if p0 != p1 {
        return 1;
    }
    if p0 != std::ptr::null() {
        return 2;
    }

    if v0 != std::ptr::null() {
        return 3;
    }
    if v0 != p0 as *const std::ffi::c_void {
        return 4;
    }

    if fp0 != fp1 {
        return 5;
    }
    if fp1 != fp2 {
        return 6;
    }
    if fp0 != Some(f) {
        return 7;
    }

    if p0 != fp0.map(|x| x as *const i32) {
        return 8;
    }

    0
}