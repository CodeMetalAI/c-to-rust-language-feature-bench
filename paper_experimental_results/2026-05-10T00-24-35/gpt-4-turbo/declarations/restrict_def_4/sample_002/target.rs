fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
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
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    y_snapshot.copy_from_slice(&y);

    let (p1, q1) = (&mut x[..], &y[..]);

    if true { // gate4 is always 1
        let (p2, q2) = (p1, q1);

        copy_restrict(16, &mut p2[8..24], &q2[4..20]);

        let expect: Vec<_> = y_snapshot[4..20].to_vec();
        if !same(&x[8..24], &expect) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot) {
            std::process::exit(2);
        }

        copy_restrict(8, &mut p1[0..8], &q1[24..32]);

        let expect2: Vec<_> = y_snapshot[24..32].to_vec();
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
}