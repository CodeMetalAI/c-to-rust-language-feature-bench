fn main() {
    let mut gate4: i32 = 1;
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    {
        let mut p1 = &mut x[..];
        let mut q1 = &mut y[..];

        if gate4!= 0 {
            let mut p2 = &mut p1[8..];
            let mut q2 = &mut q1[4..];

            copy_restrict(16, p2, q2);

            let expect: Vec<i32> = y_snapshot[4..20].iter().cloned().collect();
            if!same(&x[8..24], &expect) {
                return 1;
            }

            if!same(&y, &y_snapshot) {
                return 2;
            }
        }

        copy_restrict(8, &mut p1[..8], &mut q1[24..32]);

        let expect2: Vec<i32> = y_snapshot[24..32].iter().cloned().collect();
        if!same(&x[..8], &expect2) {
            return 3;
        }
    }

    if!same(&y, &y_snapshot) {
        return 4;
    }

    if sum(&x) == 0 {
        return 5;
    }

    return 0;
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i as i32 * 5 + 1;
    }
}

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    a.iter().zip(b.iter()).take(n).all(|(x, y)| x == y)
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum::<i32>()
}