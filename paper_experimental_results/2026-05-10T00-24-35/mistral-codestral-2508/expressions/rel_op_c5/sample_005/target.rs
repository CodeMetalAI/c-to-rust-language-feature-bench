struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}

fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if !std::ptr::eq(px1, px2) {
        std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3];
    let p_end2 = &arr[3];
    if !std::ptr::eq(p_end1, p_end2) {
        std::process::exit(2);
    }

    let p0 = &arr[0];
    let p2 = &arr[2];
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = &arr[2];
    let q1 = q_last.wrapping_add(1);
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as *const u8;
    let sb = &s.b as *const i32 as *const u8;
    if !(sb > sa) {
        std::process::exit(6);
    }

    let u = Un { i: 0 };
    let ui = &u.i as *const i32 as *const u8;
    let ud = &u.d as *const f64 as *const u8;
    if !std::ptr::eq(ui, ud) {
        std::process::exit(7);
    }

    std::process::exit(0);
}