fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;

    let pf: fn(i32) -> i32 = f;

    let n1: Option<i32> = None;
    let v0: *mut u8 = std::ptr::null_mut();
    let n2: Option<i32> = unsafe {
        let ptr = v0 as *mut i32;
        Some(*ptr)
    };

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: *mut u8 = std::ptr::null_mut();
    let nf2: Option<fn(i32) -> i32> = unsafe {
        let ptr = vf0 as *mut fn(i32) -> i32;
        Some(*ptr)
    };

    if n1.is_some() {
        return 1;
    }
    if n2.is_some() {
        return 2;
    }
    if v0 != std::ptr::null_mut() {
        return 3;
    }

    if n1.map_or(false, |n| &n as *const i32 == px) {
        return 4;
    }
    if n2.map_or(false, |n| &n as *const i32 == px) {
        return 5;
    }

    if nf1.is_some() {
        return 6;
    }
    if nf2.is_some() {
        return 7;
    }
    if nf1.map_or(false, |n| &n as *const fn(i32) -> i32 == &pf) {
        return 8;
    }
    if nf2.map_or(false, |n| &n as *const fn(i32) -> i32 == &pf) {
        return 9;
    }

    let vx: *mut i32 = px;
    let vf: *mut fn(i32) -> i32 = pf as *mut fn(i32) -> i32;

    if vx == std::ptr::null_mut() {
        return 10;
    }
    if vf == std::ptr::null_mut() {
        return 11;
    }
    if vx as *mut u8 == v0 {
        return 12;
    }
    if vf as *mut u8 == vf0 {
        return 13;
    }

    0
}