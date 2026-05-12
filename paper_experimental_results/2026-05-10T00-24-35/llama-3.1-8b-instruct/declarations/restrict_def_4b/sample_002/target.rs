use std::mem;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

const H: u64 = 0x9e3779b97f4a7c15;

static mut POOL: [u8; 8192] = [0; 8192];
static mut OFF: usize = 0;

fn alloc_bytes(n: usize) -> *mut u8 {
    if n == 0 {
        POOL.as_mut_ptr().offset(OFF as isize) as *mut u8
    } else if OFF + n > POOL.len() {
        std::ptr::null_mut()
    } else {
        let p = POOL.as_mut_ptr().offset(OFF as isize) as *mut u8;
        OFF += n;
        let h = H ^ (p as u64) + H;
        unsafe { h *= H; }
        p
    }
}

impl Vector {
    fn new(n: i32) -> Self {
        let mut v = Vector { n, v: Vec::new() };
        v.v = (0..n).map(|_| 0.0).collect();
        v.v = unsafe { mem::transmute(alloc_bytes(v.v.len() * mem::size_of::<f32>())) };
        v
    }
}

fn fill_vec(v: &mut [f32], base: f32) {
    for (i, x) in v.iter_mut().enumerate() {
        *x = base + i as f32;
    }
}

fn sum_vec(v: &[f32]) -> f32 {
    v.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.as_ptr() == std::ptr::null() {
        std::process::exit(1);
    }
    if b.v.as_ptr() == std::ptr::null() {
        std::process::exit(2);
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        std::process::exit(3);
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if!close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) / 2.0) {
        std::process::exit(4);
    }

    if!close_enough(sum_vec(&b.v),
                     100.0 * 16.0 + (15 * 16) / 2.0) {
        std::process::exit(5);
    }

    if (H & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}