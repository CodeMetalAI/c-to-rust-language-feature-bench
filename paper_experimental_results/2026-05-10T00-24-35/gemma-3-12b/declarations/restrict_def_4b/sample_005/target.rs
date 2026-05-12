use std::mem::size_of;

#[derive(Debug)]
struct vector {
    n: i32,
    v: *mut f32,
}

static mut H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> *mut u8 {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        return POOL.as_mut_ptr().add(OFF);
    }

    if OFF + n > 8192 {
        return std::ptr::null_mut();
    }

    let p = POOL.as_mut_ptr().add(OFF);
    OFF += n;

    unsafe {
        H ^= (p as *const u8) as u64 + 0x9e3779b97f4a7c15;
        H *= 0x5851f42d4c957f2d;
    }

    p
}

fn new_vector(n: i32) -> vector {
    let mut t = vector {
        n,
        v: alloc_bytes(n as usize * size_of::<f32>()) as *mut f32,
    };
    t
}

fn fill_vec(p: *mut f32, n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        unsafe {
            *p.add(i as usize) = base + i as f32;
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
    let d = (x - y).abs();
    d < 0.0001f32
}

fn main() -> i32 {
    unsafe {
        let a = new_vector(16);
        let b = new_vector(16);

        if a.v.is_null() {
            return 1;
        }
        if b.v.is_null() {
            return 2;
        }

        if a.v == b.v {
            return 3;
        }

        fill_vec(a.v, a.n, 1.0f32);
        fill_vec(b.v, b.n, 100.0f32);

        if !close_enough(sum_vec(a.v, a.n), 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            return 4;
        }

        if !close_enough(sum_vec(b.v, b.n),
                         100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            return 5;
        }

        if (H & 1) == 0 {
            return 6;
        }

        0
    }
}