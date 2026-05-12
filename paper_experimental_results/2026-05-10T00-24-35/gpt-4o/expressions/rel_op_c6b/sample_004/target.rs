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
    let ps: *const () = &s as *const _ as *const ();
    let px: *const () = &s.x as *const _ as *const ();
    if ps != px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1 = a.as_ptr().wrapping_add(3);
    let end2 = unsafe { a.as_ptr().offset(3) };
    if end1 != end2 {
        return 3;
    }

    0
}