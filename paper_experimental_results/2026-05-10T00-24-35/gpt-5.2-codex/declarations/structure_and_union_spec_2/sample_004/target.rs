use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct S {
    n: i32,
}

#[repr(C)]
struct SS {
    n: i32,
}

fn make_vec(len: usize) -> Option<Vec<f64>> {
    let mut v: Vec<f64> = Vec::new();
    if v.try_reserve_exact(len).is_err() {
        return None;
    }
    v.resize(len, 0.0);
    Some(v)
}

fn main() {
    if size_of::<S>() < size_of::<SS>() {
        exit(1);
    }

    let offset_d = size_of::<S>();
    if offset_d != size_of::<S>() {
        exit(1);
    }

    let mut s1 = match make_vec(8) {
        Some(v) => v,
        None => exit(1),
    };
    let mut s2 = match make_vec(5) {
        Some(v) => v,
        None => exit(1),
    };

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        exit(1);
    }
}