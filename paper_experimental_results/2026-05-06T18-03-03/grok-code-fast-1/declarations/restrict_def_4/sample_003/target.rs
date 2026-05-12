fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + (i as i32) * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a == b
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() {
    let gate4 = true;
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    y_snapshot.copy_from_slice(&y);

    if gate4 {
        x[8..24].copy_from_slice(&y[4..20]);

        if !same(&x[8..24], &y_snapshot[4..20]) {
            std::process::exit(1);
        }

        if !same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    x[0..8].copy_from_slice(&y[24..32]);

    if !same(&x[0..8], &y_snapshot[24..32]) {
        std::process::exit(3);
    }

    if !same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}