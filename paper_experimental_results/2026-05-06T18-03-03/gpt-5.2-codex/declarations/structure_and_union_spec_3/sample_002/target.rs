use std::mem::{offset_of, size_of};
use std::process::exit;

#[repr(C)]
struct LayoutCheck {
    i: i32,
    a: [i32; 0],
}

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    if offset_of!(LayoutCheck, i) != 0 {
        exit(1);
    }

    if offset_of!(LayoutCheck, a) != size_of::<LayoutCheck>() {
        exit(2);
    }

    let mut a = Vec::new();
    if a.try_reserve_exact(4).is_err() {
        exit(3);
    }
    a.resize(4, 0);

    let mut p = S { i: 0, a };

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        exit(4);
    }
}