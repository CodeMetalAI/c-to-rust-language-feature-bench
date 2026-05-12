fn main() {
    static OPAQUE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(3);

    fn h(n: usize, p: &mut [i32], q: &[i32], r: &[i32]) {
        for i in 0..n {
            p[i] = q[i] + r[i];
        }
    }

    fn init_seq(x: &mut [i32], base: i32) {
        for i in 0..x.len() {
            x[i] = base + (i as i32) * 7 + 1;
        }
    }

    fn check_unchanged(x: &[i32], y: &[i32]) -> bool {
        x.iter().zip(y.iter()).all(|(&a, &b)| a == b)
    }

    fn check_p_is_double_q(p: &[i32], q: &[i32]) -> bool {
        p.iter().zip(q.iter()).all(|(&p_val, &q_val)| p_val == q_val + q_val)
    }

    fn touch_readonly(x: &[i32]) {
        let mut sink = 0;
        for &val in x {
            sink ^= val + OPAQUE.load(std::sync::atomic::Ordering::Relaxed);
        }
        std::sync::atomic::AtomicI32::new(sink).store(sink, std::sync::atomic::Ordering::Relaxed);
    }

    let mut a = [0i32; 64];
    let mut b = [0i32; 64];
    let mut b_snapshot = [0i32; 64];

    init_seq(&mut a, 10);
    init_seq(&mut b, 100);

    b_snapshot.copy_from_slice(&b);

    touch_readonly(&b);

    h(64, &mut a, &b, &b);

    if !check_p_is_double_q(&a, &b_snapshot) {
        std::process::exit(1);
    }

    if !check_unchanged(&b, &b_snapshot) {
        std::process::exit(2);
    }

    touch_readonly(&b);

    std::process::exit(0);
}