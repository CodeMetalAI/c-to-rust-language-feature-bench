fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + i as i32 * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a == b
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() -> i32 {
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    y_snapshot.copy_from_slice(&y);

    let gate4 = true;

    if gate4 {
        let p2 = &mut x[8..24];
        let q2 = &y[4..20];

        copy_restrict(16, p2, q2);

        let mut expect = [0i32; 16];
        for i in 0..16 {
            expect[i] = y_snapshot[4 + i];
        }
        if !same(&x[8..24], &expect) {
            return 1;
        }

        if !same(&y, &y_snapshot) {
            return 2;
        }
    }

    let p1 = &mut x[0..8];
    let q1 = &y[24..32];

    copy_restrict(8, p1, q1);

    let mut expect2 = [0i32; 8];
    for i in 0..8 {
        expect2[i] = y_snapshot[24 + i];
    }
    if !same(&x[0..8], &expect2) {
        return 3;
    }

    if !same(&y, &y_snapshot) {
        return 4;
    }

    if sum(&x) == 0 {
        return 5;
    }

    0
}