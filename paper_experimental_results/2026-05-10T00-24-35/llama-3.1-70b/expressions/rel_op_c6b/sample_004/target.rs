struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 as *const _!= np2 as *const _ {
        return 1;
    }

    let s = S { x: 0, y: 0 };
    let ps: *const _ = &s as *const _;
    let px: *const _ = &s.x as *const _;
    if ps!= px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1: *const i32 = &a[3];
    let end2: *const i32 = a.as_ptr().add(3);
    if end1!= end2 {
        return 3;
    }

    return 0;
}