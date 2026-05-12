struct S {
    x: i32,
    y: i32,
}

fn main() -> i32 {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        return 1;
    }

    let s = S { x: 0, y: 0 };
    let ps: *const u8 = &s as *const _ as *const u8;
    let px: *const u8 = &s.x as *const _ as *const u8;
    if ps != px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1 = &a[3] as *const i32;
    let end2 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        return 3;
    }

    0
}