fn main() {
    let mut x: i32 = 0;
    let px1: *const i32 = &x;
    let px2: *const i32 = &x;
    if px1!= px2 {
        return 1;
    }

    let arr: [i32; 3] = [1, 2, 3];
    let p_end1: *const i32 = &arr[3];
    let p_end2: *const i32 = arr.as_ptr().offset(3);
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

    struct St {
        a: i32,
        b: i32,
    }
    let s: St = St { a: 0, b: 0 };
    let sa: *const i32 = &s.a as *const i32;
    let sb: *const i32 = &s.b as *const i32;
    if!(sb > sa) {
        return 6;
    }

    union Un {
        i32: i32,
        _d: f64,
    }
    let u: Un = Un { i32: 0 };
    let ui: *const i32 = &u.i as *const i32;
    let ud: *const f64 = &u._d as *const f64;
    if ui!= ud {
        return 7;
    }

    return 0;
}