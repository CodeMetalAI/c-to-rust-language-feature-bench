const GATE4: bool = true;

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    p[..n].copy_from_slice(&q[..n]);
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
    let mut x: Vec<i32> = vec![0; 32];
    let mut y: Vec<i32> = vec![0; 32];
    let mut y_snapshot: Vec<i32> = vec![0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    if GATE4 {
        copy_restrict(16, &mut x[8..], &y[4..]);

        let mut expect: Vec<i32> = vec![0; 16];
        for i in 0..16 {
            expect[i] = y_snapshot[4 + i];
        }
        if !same(&x[8..], &expect, 16) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot, 32) {
            std::process::exit(2);
        }
    }

    copy_restrict(8, &mut x[0..], &y[24..]);

    let mut expect2: Vec<i32> = vec![0; 8];
    for i in 0..8 {
        expect2[i] = y_snapshot[24 + i];
    }
    if !same(&x[0..], &expect2, 8) {
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