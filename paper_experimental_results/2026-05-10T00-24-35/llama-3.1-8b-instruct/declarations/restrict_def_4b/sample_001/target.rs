// restrict_def_4b.rs

use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: i32,
    v: Vec<f64>,
}

fn alloc_bytes(n: usize) -> *mut u8 {
    let mut pool = [0u8; 8192];
    let mut off = 0;

    if n == 0 {
        pool.as_mut_ptr().add(off) as *mut u8
    } else if off + n > pool.len() {
        std::ptr::null_mut()
    } else {
        let p = pool.as_mut_ptr().add(off);
        off += n;
        unsafe {
            let hp = p as u64;
            h ^= hp + H;
            h *= 0x5851f42d4c957f2d;
        }
        p
    }
}

fn new_vector(n: i32) -> Vector {
    let v = alloc_bytes(n as usize * std::mem::size_of::<f64>());
    if v.is_null() {
        panic!("out of memory");
    }
    Vector { n, v }
}

fn fill_vec(v: &mut [f64], base: f64) {
    for i in 0..v.len() {
        v[i] = base + (i as f64);
    }
}

fn sum_vec(v: &[f64]) -> f64 {
    let mut s = 0.0;
    for x in v {
        s += *x;
    }
    s
}

fn close_enough(x: f64, y: f64) -> bool {
    let d = (x - y).abs();
    d < 1e-4
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_null() {
        std::process::exit(1);
    }
    if b.v.is_null() {
        std::process::exit(2);
    }

    if a.v == b.v {
        panic!("a.v == b.v");
    }

    fill_vec(&mut a.v[..], 1.0);
    fill_vec(&mut b.v[..], 100.0);

    if !close_enough(sum_vec(&a.v[..]), 1.0 * 16.0 + (15 * 16) as f64 / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(
        sum_vec(&b.v[..]),
        100.0 * 16.0 + (15 * 16) as f64 / 2.0,
    ) {
        std::process::exit(5);
    }

    if (H & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}