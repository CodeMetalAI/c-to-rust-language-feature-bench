fn copy_restrict(p: &mut [i32], q: &[i32]) {
    for i in 0..p.len() {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() {
    let gate4: i32 = 1;

    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    if gate4 != 0 {
        let p2 = &mut x[8..24];
        let q2 = &y[4..20];
        copy_restrict(p2, q2);

        let expect = &y_snapshot[4..20];
        if !same(&x[8..24], expect) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    let p1 = &mut x[0..8];
    let q1 = &y[24..32];
    copy_restrict(p1, q1);

    let expect2 = &y_snapshot[24..32];
    if !same(&x[0..8], expect2) {
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