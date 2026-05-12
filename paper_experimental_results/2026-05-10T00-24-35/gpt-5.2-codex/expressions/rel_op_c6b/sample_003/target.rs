#[repr(C)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps = std::ptr::from_ref(&s) as *const u8;
    let px = std::ptr::from_ref(&s.x) as *const i32 as *const u8;
    if ps != px {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let range = a.as_ptr_range();
    let end1 = range.end;
    let end2 = range.start.wrapping_add(3);
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}