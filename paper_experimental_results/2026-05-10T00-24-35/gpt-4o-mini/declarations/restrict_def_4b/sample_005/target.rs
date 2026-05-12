struct Vector {
    n: i32,
    v: Option<Box<[f32; 16]>>,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> Option<*mut u8> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    unsafe {
        if n == 0 {
            return Some(POOL.as_mut_ptr().add(OFF));
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = POOL.as_mut_ptr().add(OFF);
        OFF += n;

        H ^= p as u64 + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(p)
    }
}

fn new_vector(n: i32) -> Vector {
    let v_pointer = alloc_bytes((n as usize) * std::mem::size_of::<f32>());
    Vector {
        n,
        v: v_pointer.map(|ptr| unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr as *mut f32, n as usize)) }),
    }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n {
        p[i as usize] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    p.iter().take(n as usize).sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    (x - y).abs() < 0.0001
}

fn main() -> i32 {
    let mut a = new_vector(16);
    let mut b = new_vector(16);

    if a.v.is_none() {
        return 1;
    }
    if b.v.is_none() {
        return 2;
    }

    if std::ptr::eq(a.v.as_ref().unwrap().as_ptr(), b.v.as_ref().unwrap().as_ptr()) {
        return 3;
    }

    fill_vec(a.v.as_mut().unwrap(), a.n, 1.0);
    fill_vec(b.v.as_mut().unwrap(), b.n, 100.0);

    if !close_enough(sum_vec(a.v.as_ref().unwrap(), a.n), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(b.v.as_ref().unwrap(), b.n), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 5;
    }

    unsafe {
        if (H & 1) == 0 {
            return 6;
        }
    }

    0
}