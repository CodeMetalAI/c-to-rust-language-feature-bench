struct St {
    a: i32,
    b: i32,
}

union Un {
    i: i32,
    d: f64,
}

fn main() {
    let x: i32 = 0;
    let px1: *const i32 = &x;
    let px2: *const i32 = &x;
    if px1!= px2 {
        return 1;
    }

    let arr: [i32; 3] = [1, 2, 3];
    let p_end1: *const i32 = &arr[3];
    let p_end2: *const i32 = &arr[3];
    if p_end1!= p_end2 {
        return 2;
    }

    let p0: *const i32 = &arr[0];
    let p2: *const i32 = &arr[2];
    if!(p2 > p0) {
        return 3;
    }
    if!(p0 < p2) {
        return 4;
    }

    let q_last: *const i32 = &arr[2];
    let q1: *const i32 = q_last.offset(1);
    if!(q1 > p0) {
        return 5;
    }

    let s: St = St { a: 0, b: 0 };
    let sa: *const i32 = &s.a as *const _;
    let sb: *const i32 = &s.b as *const _;
    if!(sb > sa) {
        return 6;
    }

    let u: Un;
    let ui: *const i32 = &u.i as *const _;
    let ud: *const f64 = &u.d as *const _;
    if ui == ud {
        return 7;
    }

    assert_eq!(ui as *const f64, ud);
    assert!(ui < ud);

    return 0;
}