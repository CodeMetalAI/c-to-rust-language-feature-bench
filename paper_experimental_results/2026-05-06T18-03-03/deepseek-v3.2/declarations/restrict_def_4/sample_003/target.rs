fn main() {
    static GATE4: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

    fn copy_restrict(p: &mut [i32], q: &[i32]) {
        let n = p.len().min(q.len());
        for i in 0..n {
            p[i] = q[i];
        }
    }

    fn fill(p: &mut [i32], base: i32) {
        for (i, elem) in p.iter_mut().enumerate() {
            *elem = base + (i as i32) * 5 + 1;
        }
    }

    fn same(a: &[i32], b: &[i32]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
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

    {
        let p1 = &mut x[..];
        let q1 = &y[..];

        if GATE4.load(std::sync::atomic::Ordering::Relaxed) {
            let p2 = &mut x[..];
            let q2 = &y[..];

            copy_restrict(&mut p2[8..24], &q2[4..20]);

            let mut expect = [0; 16];
            for i in 0..16 {
                expect[i] = y_snapshot[4 + i];
            }
            if !same(&x[8..24], &expect) {
                std::process::exit(1);
            }

            if !same(&y, &y_snapshot) {
                std::process::exit(2);
            }
        }

        copy_restrict(&mut p1[0..8], &q1[24..32]);

        let mut expect2 = [0; 8];
        for i in 0..8 {
            expect2[i] = y_snapshot[24 + i];
        }
        if !same(&x[0..8], &expect2) {
            std::process::exit(3);
        }
    }

    if !same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}