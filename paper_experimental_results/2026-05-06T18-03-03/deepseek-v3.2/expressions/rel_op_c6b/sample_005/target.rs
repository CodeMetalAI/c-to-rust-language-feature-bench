struct S {
    x: i32,
    y: i32,
}

fn main() {
    // Null pointer comparison
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        std::process::exit(1);
    }

    // Struct and field pointer comparison
    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if ps != px as *const S {
        std::process::exit(2);
    }

    // Array end pointer comparison
    let a = [1, 2, 3];
    let end1 = unsafe { a.as_ptr().add(3) };
    let end2 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}