fn main() {
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        std::process::exit(1);
    }

    struct S {
        x: i32,
        y: i32,
    }

    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    // In C, the address of a struct and its first member are the same
    // We compare the raw pointer values
    if ps as usize != px as usize {
        std::process::exit(2);
    }

    let a: [i32; 3] = [1, 2, 3];
    let end1: *const i32 = a.as_ptr().wrapping_add(3);
    let end2: *const i32 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}