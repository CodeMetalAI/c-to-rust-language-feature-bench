fn main() {
    let x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1!= px2 {
        std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3] as *const i32;
    let p_end2 = arr.as_ptr().offset(3);
    if p_end1!= p_end2 {
        std::process::exit(2);
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if!p2.offset_from(p0).is_positive() {
        std::process::exit(3);
    }
    if!p0.offset_from(p2).is_negative() {
        std::process::exit(4);
    }

    let q_last = &arr[2] as *const i32;
    let q1 = q_last.offset(1);
    if!q1.offset_from(p0).is_positive() {
        std::process::exit(5);
    }

    let s = std::mem::MaybeUninit::<(i32, i32)>::uninit().assume_init();
    let sa = &s.0 as *const i32 as *const u8;
    let sb = &s.1 as *const i32 as *const u8;
    if!sb.offset_from(sa).is_positive() {
        std::process::exit(6);
    }

    let u = std::mem::MaybeUninit::<(i32, f64)>::uninit();
    let ui = &u.0 as *const i32 as *const u8;
    let ud = &u.1 as *const f64 as *const u8;
    if ui!= ud {
        std::process::exit(7);
    }

    std::process::exit(0);
}