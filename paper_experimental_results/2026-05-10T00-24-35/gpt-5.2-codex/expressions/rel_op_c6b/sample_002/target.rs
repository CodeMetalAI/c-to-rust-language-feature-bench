use std::mem::offset_of;

#[repr(C)]
struct S {
    x: i32,
    y: i32,
}

fn main() -> i32 {
    let np1: Option<()> = None;
    let np2: Option<()> = None;
    if np1 != np2 {
        return 1;
    }

    let _s = S { x: 0, y: 0 };
    let ps_equals_px = offset_of!(S, x) == 0;
    if !ps_equals_px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1 = a.len();
    let end2 = 3usize;
    if end1 != end2 {
        return 3;
    }

    0
}