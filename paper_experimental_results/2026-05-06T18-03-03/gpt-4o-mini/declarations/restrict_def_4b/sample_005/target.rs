struct Vector {
    n: i32,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Option<&'static mut [u8; 8192]> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    unsafe {
        if n == 0 {
            return Some(&mut POOL);
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = &mut POOL[OFF..OFF + n];
        OFF += n;

        H ^= p.as_ptr() as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(p)
    }
}

fn new_vector(n: i32) -> Vector {
    let v = alloc_bytes((n as usize) * std::mem::size_of::<f32>()).expect("Allocation failed");
    Vector { n, v: vec![0.0; n as usize] }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n {
        p[i as usize] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    let mut s = 0.0;
    for i in 0..n {
        s += p[i as usize];
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
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

    if !close_enough(sum_vec(&a.v, a.n), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(&b.v, b.n), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 5;
    }

    unsafe {
        if (H & 1) == 0 {
            return 6;
        }
    }

    0
}