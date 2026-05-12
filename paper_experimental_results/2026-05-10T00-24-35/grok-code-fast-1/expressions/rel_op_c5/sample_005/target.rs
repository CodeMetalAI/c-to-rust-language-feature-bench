struct St {
    a: i32,
    b: i32,
}

fn main() {
    let mut x = 0;
    let px1 = &x as *const ();
    let px2 = &x as *const ();
    if px1 != px2 {
        std::process::exit(1);
    }

    let arr: [i32; 3] = [1, 2, 3];
    let p_end1 = (&arr[2] as *const i32).wrapping_add(1) as *const ();
    let p_end2 = arr.as_ptr().wrapping_add(3) as *const ();
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = &arr[0] as *const ();
    let p2 = &arr[2] as *const ();
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = &arr[2] as *const ();
    let q1 = (q_last as *const i32).wrapping_add(1) as *const ();
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let s = St { a: 0, b: 0 };
    let sa = (&s.a as *const i32).cast::<()>();
    let sb = (&s.b as *const i32).cast::<()>();
    if !(sb > sa) {
        std::process::exit(6);
    }

    // Union pointers are equal in address, so skip return 7

    std::process::exit(0);
}