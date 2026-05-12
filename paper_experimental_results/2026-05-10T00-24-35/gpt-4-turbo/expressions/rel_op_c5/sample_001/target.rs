fn main() {
    let mut x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1 != px2 {
        return std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3] as *const i32;
    let p_end2 = unsafe { arr.as_ptr().add(3) };
    if p_end1 != p_end2 {
        return std::process::exit(2);
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if !(p2 > p0) {
        return std::process::exit(3);
    }
    if !(p0 < p2) {
        return std::process::exit(4);
    }

    let q_last = &arr[2] as *const i32;
    let q1 = unsafe { q_last.add(1) };
    if !(q1 > p0) {
        return std::process::exit(5);
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as *const u8;
    let sb = &s.b as *const i32 as *const u8;
    if !(sb > sa) {
        return std::process::exit(6);
    }

    let mut u = Un { i: 0 }; // Initialize one field of the union
    let ui = unsafe { &mut u.i as *mut i32 as *mut u8 };
    let ud = unsafe { &mut u.d as *mut f64 as *mut u8 };
    if ui != ud {
        return std::process::exit(7);
    }

    std::process::exit(0);
}

struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}