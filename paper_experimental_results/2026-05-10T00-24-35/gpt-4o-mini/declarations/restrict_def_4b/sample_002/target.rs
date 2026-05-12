struct Vector {
    n: i32,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> *mut u8 {
    const POOL_SIZE: usize = 8192;
    static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];
    static mut OFF: usize = 0;

    unsafe {
        if n == 0 {
            return POOL.as_mut_ptr().add(OFF);
        }

        if OFF + n > POOL_SIZE {
            return std::ptr::null_mut();
        }

        let p = POOL.as_mut_ptr().add(OFF);
        OFF += n;

        H ^= p as usize as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        p
    }
}

fn new_vector(n: i32) -> Vector {
    let v = unsafe { alloc_bytes((n as usize) * std::mem::size_of::<f32>()) as *mut f32 };
    Vector { n, v: unsafe { Vec::from_raw_parts(v, n as usize, n as usize) } }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n {
        p[i as usize] = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    (x - y).abs() < 0.0001
}

fn main() -> i32 {
    let mut a = new_vector(16);
    let mut b = new_vector(16);

    if a.v.is_empty() {
        return 1;
    }
    if b.v.is_empty() {
        return 2;
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        return 3;
    }

    fill_vec(&mut a.v, a.n, 1.0);
    fill_vec(&mut b.v, b.n, 100.0);

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 5;
    }

    unsafe {
        if (H & 1) == 0 {
            return 6;
        }
    }

    0
}