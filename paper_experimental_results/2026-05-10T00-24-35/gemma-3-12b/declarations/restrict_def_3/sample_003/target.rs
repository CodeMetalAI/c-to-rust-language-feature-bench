fn main() {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

    for i in 0..64 {
        a[i] = 10 + i * 7 + 1;
    }
    for i in 0..64 {
        b[i] = 100 + i * 7 + 1;
    }

    for i in 0..64 {
        b_snapshot[i] = b[i];
    }

    let mut sink = 0;
    for i in 0..64 {
        sink ^= b[i] + 3;
    }

    for i in 0..64 {
        a[i] = b[i] + b[i];
    }

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        return 1;
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        return 2;
    }

    let mut sink = 0;
    for i in 0..64 {
        sink ^= b[i] + 3;
    }

    0
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> bool {
    for i in 0..n {
        if p[i] != q[i] + q[i] {
            return false;
        }
    }
    true
}

fn check_unchanged(x: &[i32], y: &[i32], n: usize) -> bool {
    for i in 0..n {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}