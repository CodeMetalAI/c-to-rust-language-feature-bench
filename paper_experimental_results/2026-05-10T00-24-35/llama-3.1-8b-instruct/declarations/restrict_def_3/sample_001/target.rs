static MUTABLE_OPAQUE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(3);

fn h(p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..p.len() {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        x[i] = base + i * 7 + 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32], n: usize) -> bool {
    for i in 0..n {
        if x[i]!= y[i] {
            return false;
        }
    }
    true
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> bool {
    for i in 0..n {
        if p[i]!= q[i] + q[i] {
            return false;
        }
    }
    true
}

fn touch_readonly(x: &[i32], n: usize) {
    let mut sink: i32 = 0;
    for i in 0..n {
        sink ^= x[i] + MUTABLE_OPAQUE.load(std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    let mut a = vec![0; 64];
    let mut b = vec![0; 64];
    let mut b_snapshot = b.clone();

    init_seq(&mut a, 64, 10);
    init_seq(&mut b, 64, 100);

    for i in 0..64 {
        b_snapshot[i] = b[i];
    }

    touch_readonly(&b, 64);

    h(&mut a, &b, &b);

    if!check_p_is_double_q(&a, &b_snapshot, 64) {
        println!("Failed to check p is double q");
        std::process::exit(1);
    }

    if!check_unchanged(&b, &b_snapshot, 64) {
        println!("Failed to check unchanged");
        std::process::exit(2);
    }

    touch_readonly(&b, 64);

    std::process::exit(0);
}