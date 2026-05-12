fn main() {
    let mut x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1 != px2 {
        return 1;
    }

    let arr = [1, 2, 3];
    let p_end1 = (&arr[3] as *const i32);
    let p_end2 = (arr.as_ptr().add(3) as *const i32);
    if p_end1 != p_end2 {
        return 2;
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if !(*p2 > *p0) {
        return 3;
    }
    if !(*p0 < *p2) {
        return 4;
    }

    let q_last = &arr[2] as *const i32;
    let q1 = (q_last.add(1) as *const i32);
    if !(*q1 > *p0) {
        return 5;
    }

    struct st {
        a: i32,
        b: i32,
    }
    let s = st { a: 0, b: 0 };
    let sa = (s.a as *const i32 as *const u8);
    let sb = (s.b as *const i32 as *const u8);
    if !(*sb > *sa) {
        return 6;
    }

    union un {
        i: i32,
        d: f64,
    }
    let u = un { i: 0 };
    let ui = (u.i as *const i32 as *const u8);
    let ud = (u.d as *const f64 as *const u8);
    if ui != ud {
        return 7;
    }

    0
}