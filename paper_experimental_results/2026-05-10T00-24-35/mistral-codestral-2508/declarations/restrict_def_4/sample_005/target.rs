fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    p.copy_from_slice(&q[..n]);
}

fn fill(p: &mut [i32], base: i32) {
    for (i, item) in p.iter_mut().enumerate() {
        *item = base + (i as i32) * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a == b
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    y_snapshot.copy_from_slice(&y);

    let gate4 = true;

    if gate4 {
        let (p2, q2) = (x[8..].split_at_mut(16).0, &y[4..]);

        copy_restrict(16, p2, q2);

        let expect = &y_snapshot[4..20];
        if !same(&x[8..24], expect) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    let (p1, q1) = (x[..8].split_at_mut(8).0, &y[24..]);

    copy_restrict(8, p1, q1);

    let expect2 = &y_snapshot[24..32];
    if !same(&x[..8], expect2) {
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