static GATE4: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i as i32 * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn sum(a: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    s
}

fn main() -> i32 {
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    y_snapshot.copy_from_slice(&y);

    let p1 = &mut x[..];
    let q1 = &y[..];

    if GATE4.load(std::sync::atomic::Ordering::Relaxed) {
        let p2 = &mut p1[8..];
        let q2 = &q1[4..];

        copy_restrict(16, p2, q2);

        let expect: Vec<i32> = y_snapshot[4..20].to_vec();
        if !same(&x[8..24], &expect, 16) {
            return 1;
        }

        if !same(&y, &y_snapshot, 32) {
            return 2;
        }
    }

    copy_restrict(8, &mut p1[0..], &q1[24..]);

    let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
    if !same(&x[0..8], &expect2, 8) {
        return 3;
    }

    if !same(&y, &y_snapshot, 32) {
        return 4;
    }

    if sum(&x, 32) == 0 {
        return 5;
    }

    0
}