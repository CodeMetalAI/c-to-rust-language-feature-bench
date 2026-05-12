fn main() {
    let x: i32 = 0;
    let px1: &i32 = &x;
    let px2: &i32 = &x;
    if px1!= px2 {
        panic!("px1!= px2");
    }

    let arr = [1, 2, 3];
    let p_end1: *const i32 = &arr[3];
    let p_end2: *const i32 = &arr[3];
    if p_end1!= p_end2 {
        panic!("p_end1!= p_end2");
    }

    let p0: *const i32 = &arr[0];
    let p2: *const i32 = &arr[2];
    if!(p2 > p0) {
        panic!("p2 <= p0");
    }
    if!(p0 < p2) {
        panic!("p0 >= p2");
    }

    let q_last: *const i32 = &arr[2];
    let q1: *const i32 = unsafe { q_last.offset(1) };
    if!(q1 > p0) {
        panic!("q1 <= p0");
    }

    struct S {
        a: i32,
        b: i32,
    }

    let s = S { a: 0, b: 0 };
    let sa: *const i32 = &s.a as *const _;
    let sb: *const i32 = &s.b as *const _;
    if!(sb > sa) {
        panic!("sb <= sa");
    }

    union U {
        i32: i,
        f64: d,
    }

    let u = U { i: 0 };
    let ui: *const i32 = &u.i as *const _;
    let ud: *const f64 = &u.d as *const _;
    if ui!= ud {
        panic!("ui!= ud");
    }
}