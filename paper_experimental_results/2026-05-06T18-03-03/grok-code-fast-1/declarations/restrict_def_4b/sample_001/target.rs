use std::cell::RefCell;

mod allocator {
    use std::cell::RefCell;

    thread_local! {
        static TOTAL: RefCell<usize> = RefCell::new(0);
        static H: RefCell<u64> = RefCell::new(0x9e3779b97f4a7c15u64);
    }

    pub fn alloc_bytes(n: usize) -> bool {
        if n == 0 {
            return true;
        }
        let success = TOTAL.with(|total| {
            let mut total = total.borrow_mut();
            if *total + n > 8192 {
                false
            } else {
                *total += n;
                true
            }
        });
        if success {
            H.with(|h| {
                let mut h = h.borrow_mut();
                *h ^= 0u64.wrapping_add(0x9e3779b97f4a7c15u64);
                *h = h.wrapping_mul(0x5851f42d4c957f2d);
            });
        }
        success
    }

    pub fn get_h() -> u64 {
        H.with(|h| *h.borrow())
    }
}

#[derive(PartialEq)]
struct Vector {
    n: i32,
    v: Option<Vec<f32>>,
}

fn new_vector(n: i32) -> Vector {
    let success = allocator::alloc_bytes((n as usize) * 4);
    Vector {
        n,
        v: if success { Some(vec![0.0; n as usize]) } else { None },
    }
}

fn fill_vec(p: &mut [f32], base: f32) {
    for i in 0..p.len() {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    let mut s = 0.0;
    for &x in p {
        s += x;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    let d = if d < 0.0 { -d } else { d };
    d < 0.0001
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        std::process::exit(1);
    }
    if b.v.is_none() {
        std::process::exit(2);
    }

    if a.v == b.v {
        std::process::exit(3);
    }

    fill_vec(a.v.as_mut().unwrap(), 1.0);
    fill_vec(b.v.as_mut().unwrap(), 100.0);

    if !close_enough(sum_vec(a.v.as_ref().unwrap()), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(b.v.as_ref().unwrap()), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        std::process::exit(5);
    }

    if (allocator::get_h() & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}