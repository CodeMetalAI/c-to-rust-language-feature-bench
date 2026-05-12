use std::cell::Cell;

thread_local! {
    static OPAQUE: Cell<i32> = Cell::new(3);
}

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
    let sink: Cell<i32> = Cell::new(0);
    let mut i = 0;
    while i < n {
        OPAQUE.with(|opaque| {
            sink.set(sink.get() ^ (x[i] + opaque.get()));
        });
        i += 1;
    }
    // Force the sink to be "used" to prevent optimization
    std::hint::black_box(sink.get());
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let mut a: [i32; 64] = [0; 64];
    let mut b: [i32; 64] = [0; 64];
    let mut b_snapshot: [i32; 64] = [0; 64];

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

    // In C, h is called with b passed twice (as both q and r)
    // Since we can't have multiple borrows, we need to clone b for the operation
    let b_copy = b.clone();
    h(64, &mut a, &b_copy, &b_copy);

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        return 1;
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        return 2;
    }

    touch_readonly(&b, 64);

    0
}