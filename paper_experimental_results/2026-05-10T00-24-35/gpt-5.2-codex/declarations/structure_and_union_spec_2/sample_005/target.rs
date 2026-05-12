use std::mem::{align_of, size_of};
use std::process::exit;

struct S {
    n: i32,
    d: Vec<f64>,
}

fn align_up(x: usize, align: usize) -> usize {
    if align == 0 {
        x
    } else {
        (x + align - 1) / align * align
    }
}

fn make_s(len: usize) -> Option<S> {
    let mut d: Vec<f64> = Vec::new();
    if d.try_reserve_exact(len).is_err() {
        return None;
    }
    d.resize(len, 0.0);
    Some(S { n: 0, d })
}

fn main() {
    let size_ss = size_of::<i32>();
    let size_s = align_up(size_of::<i32>(), align_of::<f64>());
    let offset_d = size_s;

    if size_s < size_ss {
        exit(1);
    }

    if offset_d != size_s {
        exit(1);
    }

    let mut s1 = match make_s(8) {
        Some(v) => v,
        None => exit(1),
    };
    let mut s2 = match make_s(5) {
        Some(v) => v,
        None => exit(1),
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        exit(1);
    }

    exit(0);
}