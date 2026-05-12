// restrict_def_4.rs
const GATE4: bool = true;

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
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
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn sum(a: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    s
}

fn main() -> i32 {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    y_snapshot.copy_from_slice(&y);

    let (p1, q1) = (&mut x, &y);

    if GATE4 {
        let p2 = p1;
        let q2 = q1;

        copy_restrict(16, &mut p2[8..], &q2[4..]);

        let expect: [i32; 16] = y_snapshot[4..20].try_into().unwrap();
        if !same(&p1[8..], &expect, 16) {
            return 1;
        }

        if !same(q1, &y_snapshot, 32) {
            return 2;
        }
    }

    copy_restrict(8, &mut p1[0..], &q1[24..]);

    let expect2: [i32; 8] = y_snapshot[24..32].try_into().unwrap();
    if !same(&p1[0..8], &expect2, 8) {
        return 3;
    }

    if !same(q1, &y_snapshot, 32) {
        return 4;
    }

    if sum(&x, 32) == 0 {
        return 5;
    }

    0
}