// restrict_def_3.rs
static mut OPAQUE: i32 = 3;

fn h(n: usize, p: &mut [i32], q: &mut [i32], r: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i] + r[i];
    }
}

fn init_seq(x: &mut [i32], n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        x[i] = base + i * 7 + 1;
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
        sink ^= x[i] + OPAQUE;
        i += 1;
    }
}

fn main() {
    let mut a = [0; 64];
    let mut b = [0; 64];
    let mut b_snapshot = [0; 64];

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

    h(64, &mut a, &mut b, &mut b);

    if !check_p_is_double_q(&a, &b_snapshot, 64) {
        panic!("check_p_is_double_q failed");
    }

    if !check_unchanged(&b, &b_snapshot, 64) {
        panic!("check_unchanged failed");
    }

    touch_readonly(&b, 64);

    panic!("main completed successfully");
}