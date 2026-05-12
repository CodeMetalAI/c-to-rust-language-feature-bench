// restrict_def_3.rs
static mut OPAQUE: i32 = 3;

fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], base: i32) {
    for (i, item) in x.iter_mut().enumerate() {
        *item = base + i as i32 * 7 + 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32]) -> bool {
    x.iter().zip(y.iter()).all(|(a, b)| a == b)
}

fn check_p_is_double_q(p: &[i32], q: &[i32]) -> bool {
    p.iter().zip(q.iter()).all(|(pi, qi)| *pi == *qi + *qi)
}

fn touch_readonly(x: &[i32]) {
    let mut sink = 0;
    for &item in x.iter() {
        unsafe {
            sink ^= item + OPAQUE;
        }
    }
    // Use sink to prevent optimization
    std::hint::black_box(sink);
}

fn main() -> i32 {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

    init_seq(&mut a, 10);
    init_seq(&mut b, 100);

    b_snapshot.copy_from_slice(&b);

    touch_readonly(&b);

    h(64, &mut a, &b, &b);

    if !check_p_is_double_q(&a, &b_snapshot) {
        return 1;
    }

    if !check_unchanged(&b, &b_snapshot) {
        return 2;
    }

    touch_readonly(&b);

    0
}