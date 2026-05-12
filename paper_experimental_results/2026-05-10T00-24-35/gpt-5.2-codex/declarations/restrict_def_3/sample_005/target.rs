use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};
use std::hint::black_box;

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
    let mut sink: i32 = 0;
    let mut i = 0;
    while i < n {
        sink ^= x[i] + OPAQUE.load(Ordering::Relaxed);
        i += 1;
    }
    black_box(sink);
}

fn main() {
    const N: usize = 64;
    let mut a = [0i32; N];
    let mut b = [0i32; N];
    let mut b_snapshot = [0i32; N];

    init_seq(&mut a, N, 10);
    init_seq(&mut b, N, 100);

    {
        let mut i = 0;
        while i < N {
            b_snapshot[i] = b[i];
            i += 1;
        }
    }

    touch_readonly(&b, N);

    h(N, &mut a, &b, &b);

    if !check_p_is_double_q(&a, &b_snapshot, N) {
        exit(1);
    }

    if !check_unchanged(&b, &b_snapshot, N) {
        exit(2);
    }

    touch_readonly(&b, N);

    exit(0);
}