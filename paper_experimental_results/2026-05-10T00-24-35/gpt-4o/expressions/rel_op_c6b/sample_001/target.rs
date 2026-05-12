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
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if (ps as *const i32) != px {
        return 2;
    }

    let a: [i32; 3] = [1, 2, 3];
    let end1 = unsafe { a.as_ptr().add(3) };
    let end2 = unsafe { a.as_ptr().offset(3) };
    if end1 != end2 {
        return 3;
    }

    0
}