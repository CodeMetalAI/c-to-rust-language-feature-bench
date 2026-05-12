use std::mem;

const H: usize = 0x9e3779b97f4a7c15;

struct Vector {
    n: usize,
    v: Vec<f32>,
}

fn alloc_bytes(n: usize) -> Option<Vec<u8>> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        unsafe { return Some(Vec::from(&POOL[OFF..])); }
    }

    unsafe {
        if OFF + n > POOL.len() {
            return None;
        }

        let p = &POOL[OFF..OFF + n];
        OFF += n;

        let mut h = H;
        h ^= p.as_ptr() as usize + H;
        h = h.wrapping_mul(0x5851f42d4c957f2d);

        Some(Vec::from(p))
    }
}

fn new_vector(n: usize) -> Option<Vector> {
    let bytes = alloc_bytes(n * mem::size_of::<f32>())?;
    let mut v = Vec::with_capacity(n);
    unsafe {
        v.set_len(n);
    }
    let v_ptr = v.as_mut_ptr();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const f32, v_ptr, n);
    }
    Some(Vector { n, v })
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
    let d = x - y;
    d.abs() < 0.0001
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

    let h = unsafe { H };
    if (h & 1) == 0 {
        std::process::exit(6);
    }
}