use std::f32;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

const H: u64 = 0x9e3779b97f4a7c15;

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

    H ^= (H ^ (p as u64) + 0x9e3779b97f4a7c15) as u64;
    H *= 0x5851f42d4c957f2d;

    p
}

fn new_vector(n: i32) -> Vector {
    let v: Vec<f32> = Vec::from(alloc_bytes(n as usize * std::mem::size_of::<f32>()));

    Vector {
        n,
        v: v
            .iter()
            .map(|_| f32::from_bits(f32::to_bits(1.0) + f32::to_bits((n as i32 - 1) as f32)))
            .collect(),
    }
}

fn fill_vec(p: &mut [f32], base: f32) {
    for i in 0..p.len() {
        p[i] = base + (i as f32);
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    if d < 0.0 {
        -(d as f32)
    } else {
        d
    } < 0.0001
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        std::process::exit(1);
    }
    if b.v.is_none() {
        std::process::exit(2);
    }

    if a.v.as_slice().ptr() == b.v.as_slice().ptr() {
        std::process::exit(3);
    }

    fill_vec(&mut a.v.take().unwrap(), 1.0);
    fill_vec(&mut b.v.take().unwrap(), 100.0);

    let a_sum = sum_vec(&a.v.take().unwrap());
    let expected_a_sum = 1.0 * 16.0 + (15 * 16.0) / 2.0;
    if !close_enough(a_sum, expected_a_sum) {
        std::process::exit(4);
    }

    let b_sum = sum_vec(&b.v.take().unwrap());
    let expected_b_sum = 100.0 * 16.0 + (15 * 16.0) / 2.0;
    if !close_enough(b_sum, expected_b_sum) {
        std::process::exit(5);
    }

    if (H & 1) == 0 {
        std::process::exit(6);
    }
    std::process::exit(0);
}