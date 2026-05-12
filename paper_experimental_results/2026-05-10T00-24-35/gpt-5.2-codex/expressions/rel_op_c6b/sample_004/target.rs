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
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: *const u8 = (&s as *const S).cast();
    let px: *const u8 = (&s.x as *const i32).cast();
    if ps != px {
        std::process::exit(2);
    }

    let a = [1i32, 2, 3];
    let base = a.as_ptr();
    let end1 = base.wrapping_add(3);
    let end2 = base.wrapping_add(3);
    if end1 != end2 {
        std::process::exit(3);
    }
}