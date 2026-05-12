use std::ptr;
use std::process;

#[derive(Debug)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = ptr::null();
    let np2: *const i32 = ptr::null();
    if np1 != np2 {
        process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if (ps as usize) != (px as usize) {
        process::exit(2);
    }

    let a = [1i32, 2, 3];
    let end1: *const i32 = a.as_ptr().wrapping_add(3);
    let end2: *const i32 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        process::exit(3);
    }

    process::exit(0);
}