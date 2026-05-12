fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        x[i] = base + i as i32 * 7 + 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32], n: usize) -> bool {
    x.iter().zip(y.iter()).all(|(a, b)| a == b)
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> bool {
    p.iter().zip(q.iter()).all(|(a, b)| a == b * 2)
}

fn touch_readonly(x: &[i32], n: usize) {
    let mut sink = 0;
    for i in 0..n {
        sink ^= x[i] + 3;
    }
}

fn main() {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

    init_seq(&mut a, 64, 10);
    init_seq(&mut b, 64, 100);

    b_snapshot.copy_from_slice(&b);

    touch_readonly(&b, 64);

    h(64, &mut a, &b, &b);

    if!check_p_is_double_q(&a, &b_snapshot, 64) {
        std::process::exit(1);
    }

    if!check_unchanged(&b, &b_snapshot, 64) {
        std::process::exit(2);
    }

    touch_readonly(&b, 64);
}