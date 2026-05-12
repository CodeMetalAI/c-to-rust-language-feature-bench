struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        return 1;
    }

    let mut s = S { x: 0, y: 0 };
    let ps = &s as *const _;
    let px = &s.x as *const _;
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