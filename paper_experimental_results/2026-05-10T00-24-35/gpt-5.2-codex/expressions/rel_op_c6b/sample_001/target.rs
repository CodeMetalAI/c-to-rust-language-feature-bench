use std::process::exit;
use std::ptr;

#[repr(C)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = ptr::null();
    let np2: *const i32 = ptr::null();
    if np1 != np2 {
        exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: *const u8 = &s as *const S as *const u8;
    let px: *const u8 = &s.x as *const i32 as *const u8;
    if ps != px {
        exit(2);
    }

    let a = [1, 2, 3];
    let end1: *const i32 = a.as_ptr().wrapping_add(3);
    let end2: *const i32 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        exit(3);
    }

    exit(0);
}