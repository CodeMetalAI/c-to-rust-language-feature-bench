use std::mem::{align_of, size_of};

#[repr(C)]
struct SHeader {
    n: i32,
    d: [f64; 0],
}

#[repr(C)]
struct SS {
    n: i32,
}

struct SData {
    n: i32,
    d: Vec<f64>,
}

fn align_to(size: usize, align: usize) -> usize {
    (size + align - 1) / align * align
}

fn main() {
    let size_s = size_of::<SHeader>();
    let size_ss = size_of::<SS>();

    if size_s < size_ss {
        std::process::exit(1);
    }

    let offset_d = align_to(size_of::<i32>(), align_of::<f64>());
    if offset_d != size_s {
        std::process::exit(1);
    }

    let mut s1 = SData { n: 0, d: Vec::new() };
    let mut s2 = SData { n: 0, d: Vec::new() };

    if s1.d.try_reserve(8).is_err() || s2.d.try_reserve(5).is_err() {
        std::process::exit(1);
    }

    for _ in 0..8 {
        s1.d.push(0.0);
    }
    for _ in 0..5 {
        s2.d.push(0.0);
    }

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}