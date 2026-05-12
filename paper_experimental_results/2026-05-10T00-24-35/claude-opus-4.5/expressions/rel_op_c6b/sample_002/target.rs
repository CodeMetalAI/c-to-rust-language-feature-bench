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
    // In C, the address of a struct equals the address of its first member
    // We compare the raw pointer values (this is safe, we're just comparing addresses)
    if ps as usize != px as usize {
        std::process::exit(2);
    }

    let a: [i32; 3] = [1, 2, 3];
    let end1: *const i32 = &a[2] as *const i32;
    let end1 = (end1 as usize + std::mem::size_of::<i32>()) as *const i32;
    let end2: *const i32 = a.as_ptr();
    let end2 = (end2 as usize + 3 * std::mem::size_of::<i32>()) as *const i32;
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}