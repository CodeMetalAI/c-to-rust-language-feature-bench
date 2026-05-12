use std::ptr;

struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let x = 0;
    let px1: *const i32 = &x;
    let px2: *const i32 = &x;
    if px1 != px2 {
        return 1;
    }

    let arr: [i32; 3] = [1, 2, 3];
    let p_end1: *const i32 = unsafe { arr.as_ptr().add(3) };
    let p_end2: *const i32 = unsafe { arr.as_ptr().add(3) };
    if p_end1 != p_end2 {
        return 2;
    }

    let p0: *const i32 = &arr[0];
    let p2: *const i32 = &arr[2];
    if !(p2 > p0) {
        return 3;
    }
    if !(p0 < p2) {
        return 4;
    }

    let q_last: *const i32 = &arr[2];
    let q1: *const i32 = unsafe { q_last.add(1) };
    if !(q1 > p0) {
        return 5;
    }

    let s = St { a: 0, b: 0 };
    let sa: *const u8 = &s.a as *const i32 as *const u8;
    let sb: *const u8 = &s.b as *const i32 as *const u8;
    if !(sb > sa) {
        return 6;
    }

    let u = Un { i: 0 };
    let ui: *const u8 = unsafe { &u.i as *const i32 as *const u8 };
    let ud: *const u8 = unsafe { &u.d as *const f64 as *const u8 };
    if ui != ud {
        return 7;
    }

    0
}