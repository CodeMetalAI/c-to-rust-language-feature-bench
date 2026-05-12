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
    (size + align - 1) & !(align - 1)
}

fn run() -> i32 {
    if size_of::<S>() < size_of::<SS>() {
        return 1;
    }

    let offset_d = align_up(size_of::<i32>(), align_of::<f64>());
    if offset_d != size_of::<S>() {
        return 1;
    }

    let mut s1: Vec<f64> = Vec::new();
    if s1.try_reserve_exact(8).is_err() {
        return 1;
    }
    s1.resize(8, 0.0);

    let mut s2: Vec<f64> = Vec::new();
    if s2.try_reserve_exact(5).is_err() {
        return 1;
    }
    s2.resize(5, 0.0);

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        return 1;
    }

    0
}

fn main() {
    exit(run());
}