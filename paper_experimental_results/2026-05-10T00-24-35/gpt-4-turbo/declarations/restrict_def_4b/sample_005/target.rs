struct Vector {
    n: usize,
    v: Vec<f32>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Vec<f32> {
    static mut OFF: usize = 0;

    const POOL_SIZE: usize = 8192;
    static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];

    if n == 0 {
        return vec![];
    }

    let actual_size = n * std::mem::size_of::<f32>();
    unsafe {
        if OFF + actual_size > POOL_SIZE {
            return vec![];
        }

        let p = &mut POOL[OFF..OFF + actual_size];
        OFF += actual_size;

        let p_address = p.as_mut_ptr() as usize;
        H ^= p_address as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        std::slice::from_raw_parts_mut(p.as_mut_ptr() as *mut f32, n).to_vec()
    }
}

fn new_vector(n: usize) -> Vector {
    Vector {
        n,
        v: alloc_bytes(n),
    }
}

fn fill_vec(p: &mut [f32], base: f32) {
    for (i, elem) in p.iter_mut().enumerate() {
        *elem = base + i as f32;
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

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + 15.0 * 16.0 / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + 15.0 * 16.0 / 2.0) {
        std::process::exit(5);
    }

    unsafe {
        if H & 1 == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}