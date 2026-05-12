use std::mem::size_of;

#[derive(Debug)]
struct vector {
    n: int,
    v: *mut f32,
}

static mut h: u64 = 0x9e3779b97f4a7c15;
static mut pool: [u8; 8192] = [0; 8192];
static mut off: usize = 0;

fn alloc_bytes(n: usize) -> *mut u8 {
    if n == 0 {
        return &mut pool[off] as *mut u8;
    }

    if off + n > pool.len() {
        return std::ptr::null_mut();
    }

    let p = &mut pool[off] as *mut u8;
    off += n;

    unsafe {
        h ^= (p as *const u8 as usize) as u64 + 0x9e3779b97f4a7c15;
        h *= 0x5851f42d4c957f2d;
    }

    p
}

fn new_vector(n: int) -> vector {
    let mut t = vector {
        n,
        v: alloc_bytes(n as usize * size_of::<f32>()) as *mut f32,
    };
    t
}

fn fill_vec(p: *mut f32, n: int, base: f32) {
    let mut i = 0;
    while i < n {
        unsafe {
            *p.add(i) = base + i as f32;
        }
        i += 1;
    }
}

fn sum_vec(p: *const f32, n: int) -> f32 {
    let mut s = 0.0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += *p.add(i);
        }
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
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

        fill_vec(a.v, a.n, 1.0);
        fill_vec(b.v, b.n, 100.0);

        if !close_enough(sum_vec(a.v, a.n), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
            return 4;
        }

        if !close_enough(sum_vec(b.v, b.n),
                         100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
            return 5;
        }

        if (h & 1) == 0 {
            return 6;
        }

        0
    }
}