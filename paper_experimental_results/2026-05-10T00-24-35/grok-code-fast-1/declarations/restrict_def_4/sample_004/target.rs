const GATE4: bool = true;

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    p[0..n].copy_from_slice(&q[0..n]);
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + i as i32 * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    &a[0..n] == &b[0..n]
}

fn sum(a: &[i32], n: usize) -> i32 {
    a[0..n].iter().sum()
}

fn main() {
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    let y_snapshot = y.clone();

    if GATE4 {
        copy_restrict(16, &mut x[8..24], &y[4..20]);

        if !same(&x[8..24], &y_snapshot[4..20], 16) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot, 32) {
            std::process::exit(2);
        }
    }

    copy_restrict(8, &mut x[0..8], &y[24..32]);

    if !same(&x[0..8], &y_snapshot[24..32], 8) {
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