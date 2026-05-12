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

    let nf1: fn(i32) -> i32 = std::ptr::null();
    let vf0: *mut std::ffi::c_void = std::ptr::null_mut();
    let nf2 = unsafe { std::mem::transmute(vf0) };

    if n1.is_null() {
        return;
    }
    if n2.is_null() {
        return;
    }
    if v0.is_null() {
        return;
    }

    if n1 == px {
        return;
    }
    if n2 == px {
        return;
    }

    if nf1.is_null() {
        return;
    }
    if nf2.is_null() {
        return;
    }
    if nf1 == pf {
        return;
    }
    if nf2 == pf {
        return;
    }

    let vx = px as *mut std::ffi::c_void;
    let vf = pf as *mut std::ffi::c_void;

    if vx.is_null() {
        return;
    }
    if vf.is_null() {
        return;
    }
    if vx == v0 {
        return;
    }
    if vf == vf0 {
        return;
    }

    0
}