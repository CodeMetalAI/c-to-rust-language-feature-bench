use std::ptr;

#[repr(C)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1 = ptr::null::<i32>();
    let np2 = ptr::null::<i32>();
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps = &s as *const S as *const u8;
    let px = &s.x as *const i32 as *const u8;
    if ps != px {
        std::process::exit(2);
    }

    let a = [1i32, 2, 3];
    let end1 = a.as_ptr().wrapping_add(3);
    let end2 = a.as_ptr().wrapping_add(3);
    if end1 != end2 {
        std::process::exit(3);
    }
}