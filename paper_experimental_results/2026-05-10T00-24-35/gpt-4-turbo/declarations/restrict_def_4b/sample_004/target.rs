struct Vector {
    n: i32,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Vec<f32> {
    static mut POOL: Vec<u8> = Vec::new();
    static mut OFF: usize = 0;

    unsafe {
        if POOL.len() == 0 {
            POOL.resize(8192, 0);
        }

        if n == 0 {
            return vec![];
        }

        if OFF + n > POOL.len() {
            return vec![];
        }

        let p = &POOL[OFF] as *const u8 as usize;
        OFF += n;

        H ^= p as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        vec![0.0; n / std::mem::size_of::<f32>()]
    }
}

fn new_vector(n: i32) -> Vector {
    Vector {
        n,
        v: alloc_bytes((n as usize) * std::mem::size_of::<f32>()),
    }
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
    (x - y).abs() < 0.0001
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

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    let expected_sum_a = 1.0 * 16.0 + 15.0 * 16.0 / 2.0;
    let expected_sum_b = 100.0 * 16.0 + 15.0 * 16.0 / 2.0;

    if !close_enough(sum_vec(&a.v), expected_sum_a) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v), expected_sum_b) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1) == 0 {
            std::process::exit(6);
        }
    }
}