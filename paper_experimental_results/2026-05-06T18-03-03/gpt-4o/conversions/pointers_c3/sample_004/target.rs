fn f(x: i32) -> i32 {
    x + 1
}

fn main() -> i32 {
    let x = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const std::ffi::c_void = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: *const std::ffi::c_void = std::ptr::null();
    let nf2: Option<fn(i32) -> i32> = None;

    if !n1.is_null() {
        return 1;
    }
    if !n2.is_null() {
        return 2;
    }
    if !v0.is_null() {
        return 3;
    }

    if n1 == px {
        return 4;
    }
    if n2 == px {
        return 5;
    }

    if nf1.is_some() {
        return 6;
    }
    if nf2.is_some() {
        return 7;
    }
    if nf1 == Some(pf) {
        return 8;
    }
    if nf2 == Some(pf) {
        return 9;
    }

    let vx: *const std::ffi::c_void = px as *const std::ffi::c_void;
    let vf: *const std::ffi::c_void = pf as *const std::ffi::c_void;

    if vx.is_null() {
        return 10;
    }
    if vf.is_null() {
        return 11;
    }
    if vx == v0 {
        return 12;
    }
    if vf == vf0 {
        return 13;
    }

    0
}