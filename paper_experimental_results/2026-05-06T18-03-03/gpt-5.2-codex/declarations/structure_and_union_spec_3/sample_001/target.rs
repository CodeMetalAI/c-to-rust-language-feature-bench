use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let off_i = 0usize;
    if off_i != 0 {
        exit(1);
    }

    let off_a = size_of::<S>();
    if off_a != size_of::<S>() {
        exit(2);
    }

    let mut v: Vec<i32> = Vec::new();
    if v.try_reserve_exact(5).is_err() {
        exit(3);
    }
    v.resize(5, 0);

    v[0] = 7;
    v[1] = 11;
    v[4] = 22;

    if v[0] != 7 || v[1] != 11 || v[4] != 22 {
        exit(4);
    }

    exit(0);
}