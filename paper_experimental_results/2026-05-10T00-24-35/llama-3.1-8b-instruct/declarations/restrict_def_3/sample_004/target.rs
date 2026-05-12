static mut OPAQUE: i32 = 3;

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
        sink ^= x[i] + OPAQUE;
    }
}

fn main() {
    let mut a = vec![0; 64];
    let mut b = vec![0; 64];
    let mut b_snapshot = vec![0; 64];

    init_seq(&mut a, 64, 10);
    init_seq(&mut b, 64, 100);

    {
        for (b_snapshot, b) in b_snapshot.iter_mut().zip(b.iter()) {
            *b_snapshot = *b;
        }
    }

    touch_readonly(&b, 64);

    h(64, &mut a, &b, &b);

    if!check_p_is_double_q(&a, &b_snapshot, 64) {
        panic!();
    }

    if!check_unchanged(&b, &b_snapshot, 64) {
        panic!();
    }

    touch_readonly(&b, 64);

    panic!(); // Return 0
}

fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}