struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}

fn main() -> i32 {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if px1 as *const _ != px2 as *const _ {
        return 1;
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3] as *const _; // Pointer to one past the end
    let p_end2 = arr.as_ptr().add(3); // Pointer to one past the end
    if p_end1 != p_end2 {
        return 2;
    }

    let p0 = &arr[0] as *const _;
    let p2 = &arr[2] as *const _;
    if !(p2 > p0) {
        return 3;
    }
    if !(p0 < p2) {
        return 4;
    }

    let q_last = &arr[2] as *const _;
    let q1 = unsafe { q_last.add(1) }; // Pointer arithmetic
    if !(q1 > p0) {
        return 5;
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const _ as *const u8; // Cast to byte pointer
    let sb = &s.b as *const _ as *const u8; // Cast to byte pointer
    if !(sb > sa) {
        return 6;
    }

    let u = Un { i: 0 };
    let ui = unsafe { &u.i as *const _ as *const u8 }; // Cast to byte pointer
    let ud = unsafe { &u.d as *const _ as *const u8 }; // Cast to byte pointer
    if ui != ud {
        return 7;
    }

    return 0;
}