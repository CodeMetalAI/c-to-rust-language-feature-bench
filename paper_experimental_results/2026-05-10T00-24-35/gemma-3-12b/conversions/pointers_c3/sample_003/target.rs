fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;

    let pf = f;

    let n1: Option<i32> = None;
    let v0: *mut u8 = std::ptr::null_mut();
    let n2: Option<i32> = unsafe { std::mem::transmute(v0) };

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: *mut u8 = std::ptr::null_mut();
    let nf2: Option<fn(i32) -> i32> = unsafe { std::mem::transmute(vf0) };

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
    if nf1.map_or(false, |n| n == pf) {
        return 8;
    }
    if nf2.map_or(false, |n| n == pf) {
        return 9;
    }

    let vx: *mut i32 = px;
    let vf: *mut fn(i32) -> i32 = pf;

    if vx == std::ptr::null_mut() {
        return 10;
    }
    if vf == std::ptr::null_mut() {
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