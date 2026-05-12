fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps = &s as *const S as *const ();
    let px = &s.x as *const i32 as *const ();
    if ps != px {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = &a[3] as *const i32;
    let end2 = unsafe { a.as_ptr().add(3) };
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}

struct S {
    x: i32,
    y: i32,
}