fn main() {
    static GATE4: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
    let gate4 = GATE4.load(std::sync::atomic::Ordering::Relaxed);

    fn copy_restrict(p: &mut [i32], q: &[i32]) {
        for (dst, src) in p.iter_mut().zip(q.iter()) {
            *dst = *src;
        }
    }

    fn fill(p: &mut [i32], base: i32) {
        for (i, val) in p.iter_mut().enumerate() {
            *val = base + (i as i32) * 5 + 1;
        }
    }

    fn same(a: &[i32], b: &[i32]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| x == y)
    }

    fn sum(a: &[i32]) -> i32 {
        a.iter().sum()
    }

    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    y_snapshot.copy_from_slice(&y);

    if gate4 {
        copy_restrict(&mut x[8..24], &y[4..20]);

        let expect: Vec<i32> = y_snapshot[4..20].to_vec();
        if !same(&x[8..24], &expect) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    copy_restrict(&mut x[0..8], &y[24..32]);

    let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
    if !same(&x[0..8], &expect2) {
        std::process::exit(3);
    }

    if !same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}