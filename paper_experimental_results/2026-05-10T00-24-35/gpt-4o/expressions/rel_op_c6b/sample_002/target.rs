struct S {
    x: i32,
    y: i32,
}

fn main() {
    // In Rust, there's no direct equivalent of null pointers for integers, but we can use Option.
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if ps != px as *const S {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = a.as_ptr().wrapping_offset(3);
    let end2 = unsafe { a.as_ptr().add(3) };
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}