use std::mem::{align_of, size_of};
use std::process::exit;

#[repr(C)]
struct S {
    n: i32,
    d: [f64; 0],
}

#[repr(C)]
struct SS {
    n: i32,
}

fn align_up(size: usize, align: usize) -> usize {
    (size + align - 1) / align * align
}

fn main() {
    let size_s = size_of::<S>();
    let size_ss = size_of::<SS>();

    if size_s < size_ss {
        exit(1);
    }

    let offset_d = align_up(size_of::<i32>(), align_of::<f64>());
    if offset_d != size_s {
        exit(1);
    }

    let mut s1 = vec![0.0f64; 8];
    let mut s2 = vec![0.0f64; 5];

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        exit(1);
    }

    exit(0);
}