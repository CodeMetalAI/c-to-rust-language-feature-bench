fn main() {
    let np1: *mut i32 = std::ptr::null_mut();
    let np2: *mut i32 = std::ptr::null_mut();
    if np1 != np2 {
        return 1;
    }

    struct S {
        x: i32,
        y: i32,
    }
    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if ps != px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1 = &a[3];
    let end2 = &a[3];
    if end1 != end2 {
        return 3;
    }

    0
}