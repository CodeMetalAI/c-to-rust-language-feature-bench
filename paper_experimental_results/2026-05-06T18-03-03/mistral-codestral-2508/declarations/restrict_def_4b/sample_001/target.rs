use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: usize,
    v: Vec<f32>,
}

fn alloc_bytes(n: usize) -> Option<Vec<u8>> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        unsafe { return Some(Vec::from_raw_parts(POOL.as_ptr().add(OFF) as *mut u8, 0, 0)); }
    }

    unsafe {
        if OFF + n > POOL.len() {
            return None;
        }

        let p = POOL.as_ptr().add(OFF) as *mut u8;
        OFF += n;

        let mut h = H;
        h ^= p as u64 + H;
        h = h.wrapping_mul(0x5851f42d4c957f2d);

        Some(Vec::from_raw_parts(p, n, n))
    }
}

fn new_vector(n: usize) -> Option<Vector> {
    let mut v = alloc_bytes(n * mem::size_of::<f32>())?;
    unsafe { v.set_len(n * mem::size_of::<f32>()); }
    let v = unsafe { Vec::from_raw_parts(v.as_ptr() as *mut f32, n, n) };
    Some(Vector { n, v })
}

fn fill_vec(p: &mut [f32], base: f32) {
    for (i, val) in p.iter_mut().enumerate() {
        *val = base + i as f32;
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
    let a = new_vector(16).expect("Failed to allocate vector a");
    let b = new_vector(16).expect("Failed to allocate vector b");

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

    if (unsafe { H } & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}