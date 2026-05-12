fn main() {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

    let mut i = 0;
    while i < 64 {
        a[i] = 10 + i * 7 + 1;
        i += 1;
    }

    i = 0;
    while i < 64 {
        b[i] = 100 + i * 7 + 1;
        i += 1;
    }

    i = 0;
    while i < 64 {
        b_snapshot[i] = b[i];
        i += 1;
    }

    let opaque: volatile i32 = 3;
    let mut sink: volatile i32 = 0;
    i = 0;
    while i < 64 {
        sink ^= b[i] + opaque;
        i += 1;
    }

    i = 0;
    while i < 64 {
        a[i] = b[i] + b[i];
        i += 1;
    }

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        return 1;
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        return 2;
    }

    i = 0;
    while i < 64 {
        sink ^= b[i] + opaque;
        i += 1;
    }

    0
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