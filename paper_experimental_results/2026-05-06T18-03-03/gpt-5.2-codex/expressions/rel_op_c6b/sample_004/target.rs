use std::process;

#[repr(C)]
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
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if (ps as *const u8) != (px as *const u8) {
        process::exit(2);
    }

    let a = [1i32, 2, 3];
    let base = a.as_ptr();
    let end1 = base.wrapping_add(3);
    let end2 = base.wrapping_add(3);
    if end1 != end2 {
        process::exit(3);
    }

    process::exit(0);
}