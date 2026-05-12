// restrict_def_3.rs
static mut OPAQUE: i32 = 3;

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
    for i in 0..n {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> bool {
    for i in 0..n {
        if p[i] != q[i] + q[i] {
            return false;
        }
    }
    true
}

fn touch_readonly(x: &[i32], n: usize) {
    let mut sink = 0;
    for i in 0..n {
        unsafe {
            sink ^= x[i] + OPAQUE;
        }
    }
}

fn main() -> i32 {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

    init_seq(&mut a, 64, 10);
    init_seq(&mut b, 64, 100);

    for i in 0..64 {
        b_snapshot[i] = b[i];
    }

    touch_readonly(&b, 64);

    h(64, &mut a, &b, &b);

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        return 1;
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        return 2;
    }

    touch_readonly(&b, 64);

    0
}