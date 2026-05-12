fn main() {
    // Convert null pointers to Option<*const i32> with None
    let np1: Option<*const i32> = None;
    let np2: Option<*const i32> = None;
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
    // Compare raw pointers to the same address
    if ps as *const () != px as *const () {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    // Get a pointer to one past the last element
    let end1: *const i32 = unsafe { a.as_ptr().add(3) };
    let end2: *const i32 = a.as_ptr().wrapping_add(3);
    // Compare raw pointers
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}