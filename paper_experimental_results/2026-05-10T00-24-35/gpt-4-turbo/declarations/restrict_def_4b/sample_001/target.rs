struct Vector {
    n: usize,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Option<Vec<f32>> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    unsafe {
        if n == 0 {
            return Some(Vec::new());
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = &mut POOL[OFF..OFF + n];
        OFF += n;
        
        H ^= p.as_ptr() as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(Vec::from_raw_parts(p.as_mut_ptr() as *mut f32, n / 4, n / 4))
    }
}

fn new_vector(n: usize) -> Vector {
    let allocated_space = alloc_bytes(n * 4).unwrap_or_else(Vec::new);
    Vector { n, v: allocated_space }
}

fn fill_vec(v: &mut [f32], base: f32) {
    for (i, value) in v.iter_mut().enumerate() {
        *value = base + i as f32;
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
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_empty() {
        std::process::exit(1);
    }
    if b.v.is_empty() {
        std::process::exit(2);
    }

    if std::ptr::eq(a.v.as_ptr(), b.v.as_ptr()) {
        std::process::exit(3);
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    let expected_sum_a = 1.0 * 16.0 + 15.0 * 16.0 / 2.0;
    let expected_sum_b = 100.0 * 16.0 + 15.0 * 16.0 / 2.0;

    if !close_enough(sum_vec(&a.v), expected_sum_a as f32) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v), expected_sum_b as f32) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1) == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}