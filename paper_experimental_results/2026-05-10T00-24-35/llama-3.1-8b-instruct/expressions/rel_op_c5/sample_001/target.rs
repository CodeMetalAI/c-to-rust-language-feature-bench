fn main() {
    let x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1 != px2 {
        panic!("px1 != px2");
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[..] as *const i32;
    let p_end2 = &arr[..] as *const i32;
    if p_end1 != p_end2 {
        panic!("p_end1 != p_end2");
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if !(p0 < p2) {
        panic!("p0 < p2");
    }
    if !(p2 > p0) {
        panic!("p2 > p0");
    }

    let q_last = &arr[2] as *const i32;
    let q1 = unsafe { q_last.add(1) as *const i32 };
    if !(q1 > p0) {
        panic!("q1 > p0");
    }

    struct S {
        a: i32,
        b: i32,
    }

    let s = S { a: 0, b: 0 };
    let sa = (&s.a as *const i32) as *const u8;
    let sb = (&s.b as *const i32) as *const u8;
    if !(sa < sb) {
        panic!("sa < sb");
    }

    union U {
        i: i32,
        d: f64,
    }

    let u = U { i: 0 };
    let ui = (&u.i as *const i32) as *const u8;
    let ud = (&u.d as *const f64) as *const u8;
    if ui == ud {
        panic!("ui != ud");
    }
}