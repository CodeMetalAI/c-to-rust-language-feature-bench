fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if !std::ptr::eq(np1, np2) {
        std::process::exit(1);
    }

    struct S {
        x: i32,
        y: i32,
    }

    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if !std::ptr::eq(ps as *const (), px as *const ()) {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1: *const i32 = &a[3];
    let end2 = a.as_ptr().wrapping_add(3);
    if !std::ptr::eq(end1, end2) {
        std::process::exit(3);
    }

    std::process::exit(0);
}