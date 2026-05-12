// restrict_def_4b.rs

use std::f32;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

const H: u64 = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: usize) -> *mut u8 {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        POOL.as_mut_ptr().add(OFF)
    } else if OFF + n > POOL.len() as usize {
        std::ptr::null_mut()
    } else {
        let p = POOL.as_mut_ptr().add(OFF);
        OFF += n;
        let h = (H ^ (p as u64) + H) * 0x5851f42d4c957f2d;
        std::ptr::write(p as *mut u64, h);
        p
    }
}

impl Vector {
    fn new(n: i32) -> Self {
        let v = Vec::from(alloc_bytes(n as usize * std::mem::size_of::<f32>()) as *mut f32);
        Vector { n, v }
    }

    fn fill(&mut self, base: f32) {
        self.v.iter_mut().for_each(|p| *p = base + self.v.iter().position(|_| true).unwrap() as f32);
    }

    fn sum(&self) -> f32 {
        self.v.iter().sum()
    }
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    if d < 0.0 {
        d = -d;
    }
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.is_null() {
        std::process::exit(1);
    }
    if b.v.is_null() {
        std::process::exit(2);
    }

    if a.v == b.v {
        std::process::exit(3);
    }

    a.fill(1.0);
    b.fill(100.0);

    if!close_enough(a.sum(), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(4);
    }

    if!close_enough(b.sum(), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(5);
    }

    if (H & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}