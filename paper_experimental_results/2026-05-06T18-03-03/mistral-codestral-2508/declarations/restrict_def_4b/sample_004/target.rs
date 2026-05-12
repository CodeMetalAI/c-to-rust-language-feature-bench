use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: usize,
    v: Vec<f32>,
}

fn new_vector(n: usize) -> Vector {
    Vector {
        n,
        v: vec![0.0; n],
    }
}

fn fill_vec(v: &mut [f32], base: f32) {
    for (i, val) in v.iter_mut().enumerate() {
        *val = base + i as f32;
    }
}

fn sum_vec(v: &[f32]) -> f32 {
    v.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let mut a = new_vector(16);
    let mut b = new_vector(16);

    if a.v.is_empty() {
        std::process::exit(1);
    }
    if b.v.is_empty() {
        std::process::exit(2);
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        std::process::exit(3);
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(5);
    }

    let mut h = H;
    h ^= a.v.as_ptr() as u64 + H;
    h *= 0x5851f42d4c957f2d;
    h ^= b.v.as_ptr() as u64 + H;
    h *= 0x5851f42d4c957f2d;

    if (h & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}