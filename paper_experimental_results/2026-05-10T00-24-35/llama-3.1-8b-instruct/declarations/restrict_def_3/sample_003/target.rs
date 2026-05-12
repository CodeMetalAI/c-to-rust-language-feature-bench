// restrict_def_3.rs
static mut OPAQUE: i32 = 3;

fn h(n: i32, p: &mut [i32], q: &mut [i32], r: &mut [i32]) {
    for i in 0..n {
        p[i as usize] = q[i as usize] + r[i as usize];
    }
}

fn init_seq(x: &mut [i32], n: i32, base: i32) {
    for i in 0..n {
        x[i as usize] = base + i * 7 + 1;
    }
}

fn check_unchanged(x: &[i32], y: &[i32], n: i32) -> bool {
    for i in 0..n {
        if x[i as usize]!= y[i as usize] {
            return false;
        }
    }
    true
}

fn check_p_is_double_q(p: &[i32], q: &[i32], n: i32) -> bool {
    for i in 0..n {
        if p[i as usize]!= q[i as usize] + q[i as usize] {
            return false;
        }
    }
    true
}

fn touch_readonly(x: &[i32], n: i32) {
    let mut sink = 0;
    for i in 0..n {
        sink ^= x[i as usize] + OPAQUE;
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

    h(64, &mut a, &mut b, &mut b);

    if!check_p_is_double_q(&a, &b_snapshot, 64) {
        panic!("a is not double b");
    }

    if!check_unchanged(&b, &b_snapshot, 64) {
        panic!("b was modified");
    }

    touch_readonly(&b, 64);
}