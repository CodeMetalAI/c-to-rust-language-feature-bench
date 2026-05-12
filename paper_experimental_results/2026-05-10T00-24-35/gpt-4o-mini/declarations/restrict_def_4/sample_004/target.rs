// restrict_def_4.rs

static mut GATE4: i32 = 1;

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + (i as i32) * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
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

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    let p1 = &mut x;
    let q1 = &y;

    if unsafe { GATE4 != 0 } {
        let p2 = p1;
        let q2 = q1;

        copy_restrict(16, &mut p2[8..], &q2[4..]);

        let expect: Vec<i32> = y_snapshot[4..20].to_vec();
        if !same(&p2[8..24], &expect) {
            return 1;
        }

        if !same(q1, &y_snapshot) {
            return 2;
        }
    }

    copy_restrict(8, &mut p1[0..], &q1[24..]);

    let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
    if !same(&p1[0..8], &expect2) {
        return 3;
    }

    if !same(q1, &y_snapshot) {
        return 4;
    }

    if sum(&x) == 0 {
        return 5;
    }

    return 0;
}