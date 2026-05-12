use std::sync::atomic::{AtomicI32, Ordering};

static OPAQUE: AtomicI32 = AtomicI32::new(3);

fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        x[i] = base + (i as i32) * 7 + 1;
        i += 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if x[i] != y[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if p[i] != q[i] + q[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn touch_readonly(x: &[i32], n: usize) {
    let sink = AtomicI32::new(0);
    let mut i = 0;
    while i < n {
        let old = sink.load(Ordering::SeqCst);
        sink.store(old ^ (x[i] + OPAQUE.load(Ordering::SeqCst)), Ordering::SeqCst);
        i += 1;
    }
}

fn main() {
    let mut a = [0i32; 64];
    let mut b = [0i32; 64];
    let mut b_snapshot = [0i32; 64];

    init_seq(&mut a, 64, 10);
    init_seq(&mut b, 64, 100);

    {
        let mut i = 0;
        while i < 64 {
            b_snapshot[i] = b[i];
            i += 1;
        }
    }

    touch_readonly(&b, 64);

    // In the C code, h is called with b passed as both q and r
    // We need to clone b to satisfy Rust's borrowing rules
    let b_copy = b.clone();
    h(64, &mut a, &b_copy, &b_copy);

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        std::process::exit(1);
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        std::process::exit(2);
    }

    touch_readonly(&b, 64);

    std::process::exit(0);
}