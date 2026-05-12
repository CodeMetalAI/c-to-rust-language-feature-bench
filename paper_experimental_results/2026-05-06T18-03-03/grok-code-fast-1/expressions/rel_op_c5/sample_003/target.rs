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
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1 != px2 {
        std::process::exit(1);
    }

    let arr: [i32; 3] = [1, 2, 3];
    let p_end1 = unsafe { arr.as_ptr().add(3) };
    let p_end2 = unsafe { arr.as_ptr().add(3) };
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if !((p2 as usize) > (p0 as usize)) {
        std::process::exit(3);
    }
    if !((p0 as usize) < (p2 as usize)) {
        std::process::exit(4);
    }

    let q_last = &arr[2] as *const i32;
    let q1 = unsafe { q_last.add(1) };
    if !((q1 as usize) > (p0 as usize)) {
        std::process::exit(5);
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as *const u8;
    let sb = &s.b as *const i32 as *const u8;
    if !((sb as usize) > (sa as usize)) {
        std::process::exit(6);
    }

    let u = Un { i: 0 };
    let ui = unsafe { &u.i as *const i32 as *const u8 };
    let ud = unsafe { &u.d as *const f64 as *const u8 };
    if ui != ud {
        std::process::exit(7);
    }

    std::process::exit(0);
}