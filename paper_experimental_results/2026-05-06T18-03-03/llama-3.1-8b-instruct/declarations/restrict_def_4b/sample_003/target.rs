use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

impl Vector {
    fn new(n: i32) -> Self {
        let v = vec![0.0; n as usize];
        Self { n, v }
    }
}

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

    let h = H ^ (p as u64) + H;
    h = h.wrapping_mul(H);

    p
}

fn fill_vec(p: *mut f32, n: i32, base: f32) {
    let p = p as *mut f32;
    for i in 0..n {
        unsafe { *p = base + i as f32; }
        p = p.offset(1);
    }
}

fn sum_vec(p: *const f32, n: i32) -> f32 {
    let mut s = 0.0;
    let p = p as *const f32;
    for i in 0..n {
        unsafe { s += *p; }
        p = p.offset(1);
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.as_ptr() == std::ptr::null_mut() {
        panic!("a.v is null");
    }
    if b.v.as_ptr() == std::ptr::null_mut() {
        panic!("b.v is null");
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        panic!("a.v and b.v are the same");
    }

    fill_vec(a.v.as_mut_ptr(), a.n, 1.0);
    fill_vec(b.v.as_mut_ptr(), b.n, 100.0);

    if!close_enough(sum_vec(a.v.as_ptr(), a.n), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        panic!("sum of a.v is incorrect");
    }

    if!close_enough(sum_vec(b.v.as_ptr(), b.n), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        panic!("sum of b.v is incorrect");
    }

    if (H & 1) == 0 {
        panic!("h is even");
    }
}