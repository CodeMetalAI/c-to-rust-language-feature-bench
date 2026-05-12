fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, item) in p.iter_mut().enumerate() {
        *item = base + (i as i32) * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
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

    {
        let p1 = &mut x;
        let q1 = &y;

        let gate4 = true;

        if gate4 {
            let p2 = &mut p1[8..24];
            let q2 = &q1[4..20];

            copy_restrict(16, p2, q2);

            let expect: Vec<i32> = y_snapshot[4..20].to_vec();
            if !same(&x[8..24], &expect) {
                std::process::exit(1);
            }

            if !same(&y, &y_snapshot) {
                std::process::exit(2);
            }
        }

        let p1_part = &mut p1[0..8];
        let q1_part = &q1[24..32];

        copy_restrict(8, p1_part, q1_part);

        let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
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