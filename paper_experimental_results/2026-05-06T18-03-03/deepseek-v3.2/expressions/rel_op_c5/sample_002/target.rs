fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if px1 != px2 {
        return;
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3];
    let p_end2 = &arr[3];
    if p_end1 != p_end2 {
        return;
    }

    let p0 = &arr[0];
    let p2 = &arr[2];
    if !(p2 > p0) {
        return;
    }
    if !(p0 < p2) {
        return;
    }

    let q_last = &arr[2];
    let q1 = &arr[3];
    if !(q1 > p0) {
        return;
    }

    struct St {
        a: i32,
        b: i32,
    }
    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as *const u8;
    let sb = &s.b as *const i32 as *const u8;
    if !(sb > sa) {
        return;
    }

    union Un {
        i: i32,
        d: f64,
    }
    let u = Un { i: 0 };
    let ui = &u.i as *const i32 as *const u8;
    let ud = &u.d as *const f64 as *const u8;
    if ui != ud {
        return;
    }

    std::process::exit(0);
}