use std::process;

struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: *const () = &s as *const S as *const ();
    let px: *const () = &s.x as *const i32 as *const ();
    if ps != px {
        process::exit(2);
    }

    let a: [i32; 3] = [1, 2, 3];
    let end1: *const i32 = unsafe { a.as_ptr().add(3) };
    let end2: *const i32 = unsafe { a.as_ptr().add(3) };
    if end1 != end2 {
        process::exit(3);
    }

    process::exit(0);
}