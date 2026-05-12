struct Vector {
    n: usize,
    v: Option<&'static mut [f32]>,
}

static mut H: usize = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Option<&'static mut [f32]> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    unsafe {
        if n == 0 {
            return None;
        }

        if OFF + n * std::mem::size_of::<f32>() > POOL.len() {
            return None;
        }

        let p = &mut POOL[OFF..OFF + n * std::mem::size_of::<f32>()];
        OFF += n * std::mem::size_of::<f32>();

        let p_addr = p.as_ptr() as usize;
        H ^= p_addr + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(unsafe { std::slice::from_raw_parts_mut(p.as_mut_ptr() as *mut f32, n) })
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
    p.iter().copied().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
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

    if std::ptr::eq(a.v.unwrap().as_ptr(), b.v.unwrap().as_ptr()) {
        std::process::exit(3);
    }

    if let Some(a_v) = a.v {
        fill_vec(a_v, 1.0);
        if !close_enough(sum_vec(a_v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
            std::process::exit(4);
        }
    }

    if let Some(b_v) = b.v {
        fill_vec(b_v, 100.0);
        if !close_enough(sum_vec(b_v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
            std::process::exit(5);
        }
    }

    unsafe {
        if (H & 1) == 0 {
            std::process::exit(6);
        }
    }
}