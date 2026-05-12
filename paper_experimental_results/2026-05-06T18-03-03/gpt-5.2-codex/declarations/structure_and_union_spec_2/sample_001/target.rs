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

fn alloc_vec(len: usize) -> Option<Vec<f64>> {
    let mut v = Vec::new();
    if v.try_reserve_exact(len).is_err() {
        return None;
    }
    v.resize(len, 0.0);
    Some(v)
}

fn main() {
    let size_s = size_of::<S>();
    let size_ss = size_of::<SS>();

    if size_s < size_ss {
        exit(1);
    }

    let int_size = size_of::<i32>();
    let double_align = align_of::<f64>();
    let offset = (int_size + double_align - 1) / double_align * double_align;

    if offset != size_s {
        exit(1);
    }

    let mut s1 = match alloc_vec(8) {
        Some(v) => v,
        None => exit(1),
    };
    let mut s2 = match alloc_vec(5) {
        Some(v) => v,
        None => exit(1),
    };

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        exit(1);
    }

    exit(0);
}