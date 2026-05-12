fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, elem) in p.iter_mut().enumerate() {
        *elem = base + i as i32 * 5 + 1;
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

    if true {  // gate4 is always 1 in the C++ code
        let (p1, q1) = (&mut x, &mut y);

        {
            copy_restrict(16, &mut p1[8..8 + 16], &q1[4..4 + 16]);

            let expect = &y_snapshot[4..4 + 16];
            if !same(&x[8..8 + 16], expect) {
                return std::process::exit(1);
            }

            if !same(&y, &y_snapshot) {
                return std::process::exit(2);
            }
        }

        copy_restrict(8, &mut p1[0..8], &q1[24..24 + 8]);

        let expect2 = &y_snapshot[24..24 + 8];
        if !same(&x[0..8], expect2) {
            return std::process::exit(3);
        }
    }

    if !same(&y, &y_snapshot) {
        return std::process::exit(4);
    }

    if sum(&x) == 0 {
        return std::process::exit(5);
    }

    return std::process::exit(0);
}