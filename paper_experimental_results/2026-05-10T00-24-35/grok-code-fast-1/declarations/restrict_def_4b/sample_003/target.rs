use std::cell::RefCell;

thread_local! {
    static H: RefCell<u64> = RefCell::new(0x9e3779b97f4a7c15u64);
}

#[derive(Debug)]
struct Vector {
    n: i32,
    v: Vec<f32>,
}

fn new_vector(n: i32) -> Vector {
    H.with(|h| {
        let mut hh = h.borrow_mut();
        *hh ^= 0 + 0x9e3779b97f4a7c15u64;
        *hh = hh.wrapping_mul(0x5851f42d4c957f2d);
    });
    Vector {
        n,
        v: vec![0.0; n as usize],
    }
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
    (x - y).abs() < 0.0001
}

fn main() {
    let mut a = new_vector(16);
    let mut b = new_vector(16);

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(5);
    }

    if H.with(|h| *h.borrow() & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}