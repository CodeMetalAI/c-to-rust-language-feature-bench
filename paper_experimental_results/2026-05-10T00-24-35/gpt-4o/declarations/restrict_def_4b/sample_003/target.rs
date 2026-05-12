const POOL_SIZE: usize = 8192;
static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];
static mut OFFSET: usize = 0;
static mut H: usize = 0x9e3779b97f4a7c15;

struct Vector {
    n: usize,
    v: *mut f32,
}

fn alloc_bytes(n: usize) -> *mut u8 {
    unsafe {
        if n == 0 {
            return POOL.as_mut_ptr().add(OFFSET);
        }

        if OFFSET + n > POOL_SIZE {
            return std::ptr::null_mut();
        }

        let p = POOL.as_mut_ptr().add(OFFSET);
        OFFSET += n;

        H ^= p as usize + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        p
    }
}

fn new_vector(n: usize) -> Vector {
    let size = n * std::mem::size_of::<f32>();
    let v = alloc_bytes(size) as *mut f32;
    Vector { n, v }
}

fn fill_vec(p: *mut f32, n: usize, base: f32) {
    unsafe {
        for i in 0..n {
            *p.add(i) = base + i as f32;
        }
    }
}

fn sum_vec(p: *const f32, n: usize) -> f32 {
    unsafe {
        let mut s = 0.0;
        for i in 0..n {
            s += *p.add(i);
        }
        s
    }
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0 {
        d = -d;
    }
    d < 0.0001
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_null() {
        std::process::exit(1);
    }
    if b.v.is_null() {
        std::process::exit(2);
    }

    if a.v == b.v {
        std::process::exit(3);
    }

    fill_vec(a.v, a.n, 1.0);
    fill_vec(b.v, b.n, 100.0);

    if !close_enough(
        sum_vec(a.v, a.n),
        1.0 * 16.0 + (15 * 16) as f32 / 2.0,
    ) {
        std::process::exit(4);
    }

    if !close_enough(
        sum_vec(b.v, b.n),
        100.0 * 16.0 + (15 * 16) as f32 / 2.0,
    ) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1) == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}