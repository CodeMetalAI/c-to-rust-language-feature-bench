const GATE4: bool = true;

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    let len = n.min(p.len()).min(q.len());
    for i in 0..len {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + (i as i32) * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() -> i32 {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    y_snapshot.copy_from_slice(&y);

    if GATE4 {
        let mut p1 = &mut x;
        let q1 = &y;

        let p2 = &mut p1[8..];
        let q2 = &q1[4..];

        copy_restrict(16, p2, q2);

        let expect: [i32; 16] = y_snapshot[4..20].try_into().unwrap();
        if !same(&x[8..24], &expect) {
            return 1;
        }

        if !same(&y, &y_snapshot) {
            return 2;
        }
    }

    copy_restrict(8, &mut x[0..8], &y[24..32]);

    let expect2: [i32; 8] = y_snapshot[24..32].try_into().unwrap();
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