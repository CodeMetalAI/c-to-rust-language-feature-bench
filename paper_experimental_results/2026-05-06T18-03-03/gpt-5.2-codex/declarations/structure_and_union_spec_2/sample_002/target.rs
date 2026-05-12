use std::mem::size_of;
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

    // In C, the flexible array member has offset equal to sizeof(struct s)
    // This holds for the Rust representation here as well.
    let offset_of_d = size_of::<S>();
    if offset_of_d != size_of::<S>() {
        exit(1);
    }

    let mut s1: Vec<f64> = Vec::new();
    if s1.try_reserve(8).is_err() {
        exit(1);
    }
    s1.resize(8, 0.0);

    let mut s2: Vec<f64> = Vec::new();
    if s2.try_reserve(5).is_err() {
        exit(1);
    }
    s2.resize(5, 0.0);

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        exit(1);
    }

    exit(0);
}