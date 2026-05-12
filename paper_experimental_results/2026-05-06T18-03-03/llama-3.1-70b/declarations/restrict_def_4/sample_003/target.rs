fn main() {
    let mut gate4 = 1;

    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    let mut p1 = &mut x[..];
    let mut q1 = &mut y[..];

    if gate4!= 0 {
        let mut p2 = &mut p1[8..];
        let mut q2 = &mut q1[4..];

        copy_restrict(16, p2, q2);

        let expect: Vec<i32> = y_snapshot[4..20].to_vec();
        if!same(&x[8..24], &expect) {
            return;
        }

        if!same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    let mut p1 = &mut x[..];
    let mut q1 = &mut y[..];
    copy_restrict(8, &mut p1[..8], &mut q1[24..32]);

    let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
    if!same(&x[..8], &expect2) {
        std::process::exit(3);
    }

    if!same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }
}

fn copy_restrict(n: usize, p: &mut [i32], q: &mut [i32]) {
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
    a.iter().zip(b.iter()).take(n).all(|(x, y)| x == y)
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}