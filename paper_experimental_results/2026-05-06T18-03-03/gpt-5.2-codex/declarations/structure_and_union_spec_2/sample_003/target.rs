use std::mem::{offset_of, size_of};
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

fn main() {
    if size_of::<S>() < size_of::<SS>() {
        exit(1);
    }

    if offset_of!(S, d) != size_of::<S>() {
        exit(1);
    }

    let mut s1: Vec<f64> = vec![0.0; 8];
    let mut s2: Vec<f64> = vec![0.0; 5];

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        exit(1);
    }

    exit(0);
}