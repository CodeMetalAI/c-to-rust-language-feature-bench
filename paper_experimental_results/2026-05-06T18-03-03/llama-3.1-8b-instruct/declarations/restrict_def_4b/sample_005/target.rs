// restrict_def_4b.rs

use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

impl Vector {
    fn new(n: i32) -> Self {
        let v = vec![0.0; n as usize];
        Self { n, v }
    }
}

fn alloc_bytes(n: usize) -> *mut u8 {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        return POOL.as_mut_ptr().offset(OFF as isize);
    }

    if OFF + n > POOL.len() {
        return std::ptr::null_mut();
    }

    let p = POOL.as_mut_ptr().offset(OFF as isize);
    OFF += n;

    let mut h = H;
    h ^= (p as u64) + H;
    h *= 0x5851f42d4c957f2d;
    h as *mut u8
}

fn fill_vec(p: &mut [f32], base: f32) {
    for i in 0..p.len() {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.as_ptr() == std::ptr::null_mut() {
        return 1;
    }
    if b.v.as_ptr() == std::ptr::null_mut() {
        return 2;
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        return 3;
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 5;
    }

    if (H & 1) == 0 {
        return 6;
    }

    println!("All tests passed");
}