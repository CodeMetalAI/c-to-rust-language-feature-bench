// restrict_def_4b.rs

use std::mem;

#[derive(Debug)]
struct Vector {
    n: i32,
    v: Vec<f32>,
}

const H: u64 = 0x9e3779b97f4a7c15ull;

fn alloc_bytes(n: usize) -> *mut u8 {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        return POOL.as_mut_ptr().offset(OFF as isize);
    }

    if OFF + n > POOL.len() {
        return std::ptr::null_mut();
    }

    let p = POOL.as_mut_ptr().offset(OFF as isize);
    OFF += n;

    let h = unsafe { H ^ (p as u64) + H };
    unsafe { H = h * 0x5851f42d4c957f2dull };

    p
}

fn new_vector(n: i32) -> Vector {
    Vector {
        n,
        v: Vec::from(alloc_bytes(n as usize * std::mem::size_of::<f32>()) as *mut f32),
    }
}

fn fill_vec(p: *mut f32, n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        unsafe { *p.offset(i as isize) = base + i as f32; }
        i += 1;
    }
}

fn sum_vec(p: *const f32, n: i32) -> f32 {
    let mut s = 0.0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
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

    if !close_enough(sum_vec(a.v, a.n), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(b.v, b.n), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(5);
    }

    if (H & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}