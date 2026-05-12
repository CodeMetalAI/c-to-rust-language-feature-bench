use std::hint::black_box;

const OPAQUE: i32 = 3;
const N: usize = 64;

fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        x[i] = base + (i as i32) * 7 + 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32], n: usize) -> i32 {
    for i in 0..n {
        if x[i] != y[i] {
            return 0;
        }
    }
    1
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: usize) -> i32 {
    for i in 0..n {
        if p[i] != q[i] + q[i] {
            return 0;
        }
    }
    1
}

fn touch_readonly(x: &[i32], n: usize) {
    let mut sink = 0i32;
    for i in 0..n {
        sink ^= x[i].wrapping_add(OPAQUE);
    }
    black_box(sink);
}

fn main() {
    let mut a = [0i32; N];
    let mut b = [0i32; N];
    let mut b_snapshot = [0i32; N];

    init_seq(&mut a, N, 10);
    init_seq(&mut b, N, 100);

    b_snapshot.copy_from_slice(&b);

    touch_readonly(&b, N);

    h(N, &mut a, &b, &b);

    if check_p_is_double_q(&a, &b_snapshot, N) == 0 {
        std::process::exit(1);
    }

    if check_unchanged(&b, &b_snapshot, N) == 0 {
        std::process::exit(2);
    }

    touch_readonly(&b, N);

    std::process::exit(0);
}