use std::mem;

const H: u64 = 0x9e3779b97f4a7c15;

struct Vector {
    n: i32,
    v: Vec<f32>,
}

impl Vector {
    fn new(n: i32) -> Self {
        let v = Vec::with_capacity(n as usize);
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

    let h = H ^ (p as u64) ^ H;
    h = h.wrapping_mul(H);

    p
}

fn fill_vec(p: &mut [f32], base: f32) {
    for i in 0..p.len() {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.capacity() == 0 {
        println!("Error: a.v is empty");
        return;
    }
    if b.v.capacity() == 0 {
        println!("Error: b.v is empty");
        return;
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        println!("Error: a.v and b.v point to the same memory");
        return;
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if!close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        println!("Error: sum of a.v is incorrect");
        return;
    }

    if!close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        println!("Error: sum of b.v is incorrect");
        return;
    }

    let h = unsafe { *(&H as *const u64) };
    if h & 1 == 0 {
        println!("Error: h is even");
        return;
    }

    println!("Success!");
}