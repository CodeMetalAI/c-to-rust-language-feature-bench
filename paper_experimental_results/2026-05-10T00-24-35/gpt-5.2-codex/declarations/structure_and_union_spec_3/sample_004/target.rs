use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct Inner {
    i: i32,
}

#[repr(C)]
struct S {
    inner: Inner,
    a: [i32; 0],
}

fn main() {
    let offset_i = 0usize;
    if offset_i != 0 {
        exit(1);
    }

    let offset_a = size_of::<Inner>();
    if offset_a != size_of::<S>() {
        exit(2);
    }

    let mut v: Vec<i32> = Vec::new();
    if v.try_reserve(5).is_err() {
        exit(3);
    }
    v.resize(5, 0);

    v[0] = 7;  // p->i
    v[1] = 11; // p->a[0]
    v[4] = 22; // p->a[3]

    if v[0] != 7 || v[1] != 11 || v[4] != 22 {
        exit(4);
    }

    exit(0);
}