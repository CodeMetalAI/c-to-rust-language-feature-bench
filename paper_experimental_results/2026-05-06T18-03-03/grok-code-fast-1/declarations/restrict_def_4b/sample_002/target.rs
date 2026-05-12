static mut POOL: [u8; 8192] = [0; 8192];
static mut OFF: usize = 0;
static mut H: u64 = 0x9e3779b97f4a7c15u64;

fn alloc_bytes(n: usize) -> *mut u8 {
    unsafe {
        if n == 0 {
            return POOL.as_mut_ptr().add(OFF);
        }
        if OFF + n > 8192 {
            return std::ptr::null_mut();
        }
        let p = POOL.as_mut_ptr().add(OFF);
        OFF += n;
        H ^= (p as usize as u64).wrapping_add(0x9e3779b97f4a7c15u64);
        H = H.wrapping_mul(0x5851f42d4c957f2du64);
        p
    }
}

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: *mut f32,
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    let p = alloc_bytes(bytes) as *mut f32;
    Vector { n, v: p }
}

fn fill_vec(p: *mut f32, n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        unsafe {
            *p.add(i as usize) = base + (i as f32);
        }
        i += 1;
    }
}

fn sum_vec(p: *const f32, n: i32) -> f32 {
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        unsafe {
            s += *p.add(i as usize);
        }
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0f32 {
        d = -d;
    }
    d < 0.0001f32
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

    fill_vec(a.v, a.n, 1.0f32);
    fill_vec(b.v, b.n, 100.0f32);

    if !close_enough(sum_vec(a.v, a.n), 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(b.v, b.n), 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}