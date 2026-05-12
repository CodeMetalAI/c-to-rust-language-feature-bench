fn main() {
    let mut x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1 != px2 {
        std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3] as *const i32;
    let p_end2 = unsafe { arr.as_ptr().add(3) };
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = &arr[2] as *const i32;
    let q1 = unsafe { q_last.add(1) };
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let s = (0, 0);
    let sa = &s.0 as *const i32 as *const u8;
    let sb = &s.1 as *const i32 as *const u8;
    if !(sb > sa) {
        std::process::exit(6);
    }

    let mut u = (0i32, 0.0);
    let ui = &mut u.0 as *mut i32 as *mut u8;
    let ud = &mut u.1 as *mut f64 as *mut u8;
    if ui != ud {
        std::process::exit(7);
    }
}