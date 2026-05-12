struct Vector {
    n: i32,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Option<Vec<f32>> {
    static mut POOL: Vec<u8> = Vec::new();
    static mut OFF: usize = 0;

    unsafe {
        if POOL.is_empty() {
            POOL.resize(8192, 0);
        }

        if n == 0 {
            return Some(Vec::new());
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = &POOL[OFF..OFF + n];
        OFF += n;

        H ^= p.as_ptr() as u64;
        H = H.wrapping_add(0x9e3779b97f4a7c15);
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(vec![0.0; n / std::mem::size_of::<f32>()])
    }
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    let v = alloc_bytes(bytes).unwrap_or_else(Vec::new);
    Vector { n, v }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n as usize {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    let mut s = 0.0f32;
    for i in 0..n as usize {
        s += p[i];
    }
    s
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

    fill_vec(&mut a.v, a.n, 1.0);
    fill_vec(&mut b.v, b.n, 100.0);

    if !close_enough(sum_vec(&a.v, a.n), 1.0 * 16.0 + 15.0 * 16.0 / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v, b.n), 100.0 * 16.0 + 15.0 * 16.0 / 2.0) {
        std::process::exit(5);
    }

    unsafe {
        if H & 1 == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}