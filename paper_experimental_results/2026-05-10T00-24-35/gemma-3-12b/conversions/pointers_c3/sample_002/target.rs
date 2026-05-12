fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;

    let pf: fn(i32) -> i32 = f;

    let n1: *mut i32 = std::ptr::null_mut();
    let v0: *mut std::ffi::c_void = std::ptr::null_mut();
    let n2 = unsafe { std::mem::transmute(v0) };

    let nf1: *mut fn(i32) -> i32 = std::ptr::null_mut();
    let vf0: *mut std::ffi::c_void = std::ptr::null_mut();
    let nf2 = unsafe { std::mem::transmute(vf0) };

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

    if !nf1.is_null() {
        return 6;
    }
    if !nf2.is_null() {
        return 7;
    }
    if nf1 == pf as *mut fn(i32) -> i32 {
        return 8;
    }
    if nf2 == pf as *mut fn(i32) -> i32 {
        return 9;
    }

    let vx = px as *mut std::ffi::c_void;
    let vf = pf as *mut std::ffi::c_void;

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