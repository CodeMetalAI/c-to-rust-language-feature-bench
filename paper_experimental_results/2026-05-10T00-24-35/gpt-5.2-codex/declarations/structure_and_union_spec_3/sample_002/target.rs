use std::mem::{offset_of, size_of};
use std::process::exit;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    if offset_of!(S, i) != 0 {
        exit(1);
    }

    if offset_of!(S, a) != size_of::<S>() {
        exit(2);
    }

    let mut v: Vec<i32> = Vec::new();
    if v.try_reserve_exact(1 + 4).is_err() {
        exit(3);
    }
    v.resize(1 + 4, 0);

    v[0] = 7; // p->i
    v[1] = 11; // p->a[0]
    v[4] = 22; // p->a[3]

    if v[0] != 7 || v[1] != 11 || v[4] != 22 {
        exit(4);
    }
}