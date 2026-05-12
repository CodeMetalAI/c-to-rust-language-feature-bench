use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: i32,
    v: *mut f32,
}

static mut POOL: [u8; 8192] = [0; 8192];
static mut OFF: usize = 0;

fn alloc_bytes(n: usize) -> *mut u8 {
    unsafe {
        if n == 0 {
            return POOL.as_mut_ptr().add(OFF);
        }

        if OFF + n > POOL.len() {
            return std::ptr::null_mut();
        }

        let p = POOL.as_mut_ptr().add(OFF);
        OFF += n;

        let mut h = H;
        h ^= p as u64 + H;
        h *= 0x5851f42d4c957f2d;

        p
    }
}

fn new_vector(n: i32) -> Vector {
    let v = alloc_bytes(n as usize * mem::size_of::<f32>()) as *mut f32;
    Vector { n, v }
}

fn fill_vec(p: *mut f32, n: i32, base: f32) {
    unsafe {
        for i in 0..n {
            *p.add(i as usize) = base + i as f32;
        }
    }
}

fn sum_vec(p: *const f32, n: i32) -> f32 {
    unsafe {
        let mut s = 0.0f32;
        for i in 0..n {
            s += *p.add(i as usize);
        }
        s
    }
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001f32
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    unsafe {
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

        let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
        if !close_enough(sum_vec(a.v, a.n), expected_a) {
            std::process::exit(4);
        }

        let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
        if !close_enough(sum_vec(b.v, b.n), expected_b) {
            std::process::exit(5);
        }

        let mut h = H;
        h ^= a.v as u64 + H;
        h *= 0x5851f42d4c957f2d;

        if (h & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }
}