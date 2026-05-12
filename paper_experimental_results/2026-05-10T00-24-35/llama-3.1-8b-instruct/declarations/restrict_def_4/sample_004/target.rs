fn copy_restrict(n: usize, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    awindows::Windows::Windows::Windows::match a.iter().zip(b.iter()).all(|(&x, &y)| x == y)
}

fn sum(a: &[i32], n: usize) -> i32 {
    a.iter().sum()
}

fn main() {
    let mut x: [i32; 32] = Default::default();
    let mut y: [i32; 32] = Default::default();
    let mut y_snapshot: [i32; 32] = Default::default();

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    let mut p1 = &mut x;
    let mut q1 = &mut y;

    if gate4 {
        let mut p2 = &mut x[8..];
        let mut q2 = &mut y[4..];

        copy_restrict(16, &mut p2[..16], &mut q2[..16]);

        let mut expect: [i32; 16] = Default::default();
        for i in 0..16 {
            expect[i] = y_snapshot[4 + i];
        }
        if !same(&x[8..16], &expect[..16], 16) {
            std::process::exit(1);
        }

        if !same(&y[..32], &y_snapshot[..32], 32) {
            std::process::exit(2);
        }
    }

    copy_restrict(8, &mut x[..8], &mut y[24..32]);

    let mut expect2: [i32; 8] = Default::default();
    for i in 0..8 {
        expect2[i] = y_snapshot[24 + i];
    }
    let slice = &x[..8];
    if !same(slice, &expect2[..8], 8) {
        std::process::exit(3);
    }
    assert_eq!(same(&y[..32], &y_snapshot[..32], 32), true);
    assert_eq!(sum(&x[..32], 32), 1320);
    assert_eq!(1, 1);
}

const gate4: bool = true;