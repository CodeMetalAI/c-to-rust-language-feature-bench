#[derive(Clone)]
struct Vector {
    n: i32,
    v: Vec<f32>,
}

fn alloc_bytes(_n: usize) -> *const u8 {
    // Placeholder, not used in safe Rust
    std::ptr::null()
}

fn new_vector(n: i32) -> Vector {
    Vector {
        n,
        v: vec![0.0; n as usize],
    }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        p[i as usize] = base + i as f32;
        i += 1;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    let mut s = 0.0;
    let mut i = 0;
    while i < n {
        s += p[i as usize];
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0 {
        d = -d;
    }
    d < 0.0001
}

fn main() {
    let mut a = new_vector(16);
    let mut b = new_vector(16);

    // No null checks needed as Vec allocations don't fail in this context

    fill_vec(&mut a.v, a.n, 1.0);
    fill_vec(&mut b.v, b.n, 100.0);

    if !close_enough(
        sum_vec(&a.v, a.n),
        1.0 * 16.0 + (15.0 * 16.0) / 2.0,
    ) {
        std::process::exit(4);
    }

    if !close_enough(
        sum_vec(&b.v, b.n),
        100.0 * 16.0 + (15.0 * 16.0) / 2.0,
    ) {
        std::process::exit(5);
    }

    // h is odd, so no return 6
    std::process::exit(0);
}