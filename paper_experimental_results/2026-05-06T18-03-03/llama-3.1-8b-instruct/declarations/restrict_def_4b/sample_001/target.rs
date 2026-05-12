// restrict_def_4b.rs

use std::mem;

#[derive(Debug)]
struct Vector {
    n: i32,
    v: Vec<f32>,
}

const H: u64 = 0x9e3779b97f4a7c15ull;

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

    let mut h = H ^ (p as u64) + H;
    h = h.wrapping_mul(0x5851f42d4c957f2dull);

    p
}

fn new_vector(n: i32) -> Vector {
    let v = Vec::with_capacity(n as usize);
    Vector { n, v }
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
    d < 0.0001f32
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.capacity() == 0 {
        eprintln!("Error: a.v is empty");
        std::process::exit(1);
    }
    if b.v.capacity() == 0 {
        eprintln!("Error: b.v is empty");
        std::process::exit(2);
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        eprintln!("Error: a.v and b.v point to the same memory");
        std::process::exit(3);
    }

    fill_vec(&mut a.v, 1.0f32);
    fill_vec(&mut b.v, 100.0f32);

    if!close_enough(sum_vec(&a.v), 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32) {
        eprintln!("Error: sum of a.v is not correct");
        std::process::exit(4);
    }

    if!close_enough(sum_vec(&b.v), 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32) {
        eprintln!("Error: sum of b.v is not correct");
        std::process::exit(5);
    }

    if (H & 1u64) == 0u64 {
        eprintln!("Error: H is even");
        std::process::exit(6);
    }

    println!("All tests passed");
}