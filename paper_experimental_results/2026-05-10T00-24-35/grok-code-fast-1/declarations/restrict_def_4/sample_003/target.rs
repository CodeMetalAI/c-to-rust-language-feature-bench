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

fn main() {
    let gate4 = true;
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);
    let y_snapshot = y;
    if gate4 {
        let p2 = &mut x[8..24];
        let q2 = &y[4..20];
        copy_restrict(16, p2, q2);
        let mut expect = [0i32; 16];
        for i in 0..16 {
            expect[i] = y_snapshot[4 + i];
        }
        if !same(&x[8..24], &expect, 16) {
            std::process::exit(1);
        }
        if !same(&y, &y_snapshot, 32) {
            std::process::exit(2);
        }
    }
    let p1 = &mut x[0..8];
    let q1 = &y[24..32];
    copy_restrict(8, p1, q1);
    let mut expect2 = [0i32; 8];
    for i in 0..8 {
        expect2[i] = y_snapshot[24 + i];
    }
    if !same(&x[0..8], &expect2, 8) {
        std::process::exit(3);
    }
    if !same(&y, &y_snapshot, 32) {
        std::process::exit(4);
    }
    if sum(&x, 32) == 0 {
        std::process::exit(5);
    }
    std::process::exit(0);
}