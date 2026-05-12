fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const std::ffi::c_void = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: fn(i32) -> i32 = std::ptr::null();
    let vf0: *const std::ffi::c_void = std::ptr::null();
    let nf2: fn(i32) -> i32 = vf0 as fn(i32) -> i32;

    if n1 != std::ptr::null() {
        panic!("n1 != 0");
    }
    if n2 != std::ptr::null() {
        panic!("n2 != 0");
    }
    if v0 != std::ptr::null() {
        panic!("v0 != 0");
    }

    if n1 == px {
        panic!("n1 == px");
    }
    if n2 == px {
        panic!("n2 == px");
    }

    if nf1 != std::ptr::null() {
        panic!("nf1 != 0");
    }
    if nf2 != std::ptr::null() {
        panic!("nf2 != 0");
    }
    if nf1 == pf {
        panic!("nf1 == pf");
    }
    if nf2 == pf {
        panic!("nf2 == pf");
    }

    let vx: *const std::ffi::c_void = px as *const std::ffi::c_void;
    let vf: *const std::ffi::c_void = std::mem::transmute(pf);

    if vx == std::ptr::null() {
        panic!("vx == 0");
    }
    if vf == std::ptr::null() {
        panic!("vf == 0");
    }
    if vx == v0 {
        panic!("vx == v0");
    }
    if vf == vf0 {
        panic!("vf == vf0");
    }

    assert_eq!(0, 0);
}