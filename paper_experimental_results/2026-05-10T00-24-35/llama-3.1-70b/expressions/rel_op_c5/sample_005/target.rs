struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}

fn main() {
    let mut x = 0;
    let px1 = &mut x as *mut i32;
    let px2 = &mut x as *mut i32;
    if px1 != px2 {
        return std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = arr.as_ptr().add(3);
    let p_end2 = arr.as_ptr().add(3);
    if p_end1 != p_end2 {
        return std::process::exit(2);
    }

    let p0 = arr.as_ptr();
    let p2 = p0.add(2);
    if !(p2 > p0) {
        return std::process::exit(3);
    }
    if !(p0 < p2) {
        return std::process::exit(4);
    }

    let q_last = p0.add(2);
    let q1 = q_last.add(1);
    if !(q1 > p0) {
        return std::process::exit(5);
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as *const u8;
    let sb = &s.b as *const i32 as *const u8;
    if !(sb > sa) {
        return std::process::exit(6);
    }

    let mut u = unsafe { std::mem::MaybeUninit::<Un>::uninit().assume_init() };
    let ui = &u.i as *const i32 as *const u8;
    let ud = &u.d as *const f64 as *const u8;
    if ui != ud {
        return std::process::exit(7);
    }

    return std::process::exit(0);
}