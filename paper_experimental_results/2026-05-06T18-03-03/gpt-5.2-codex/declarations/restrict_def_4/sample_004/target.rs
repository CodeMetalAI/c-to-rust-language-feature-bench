use std::sync::atomic::{AtomicI32, Ordering};

static GATE4: AtomicI32 = AtomicI32::new(1);

fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        p[i] = base + i as i32 * 5 + 1;
        i += 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn sum(a: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += a[i];
        i += 1;
    }
    s
}

fn main() {
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    {
        let mut i = 0;
        while i < 32 {
            y_snapshot[i] = y[i];
            i += 1;
        }
    }

    {
        let p1: &mut [i32] = &mut x;
        let q1: &[i32] = &y;

        if GATE4.load(Ordering::SeqCst) != 0 {
            let p2: &mut [i32] = &mut p1[..];
            let q2: &[i32] = &q1[..];

            copy_restrict(16, &mut p2[8..], &q2[4..]);

            {
                let mut expect = [0i32; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..], &expect, 16) {
                    std::process::exit(1);
                }
            }

            if !same(&y, &y_snapshot, 32) {
                std::process::exit(2);
            }
        }

        copy_restrict(8, &mut p1[0..], &q1[24..]);

        {
            let mut expect2 = [0i32; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[0..], &expect2, 8) {
                std::process::exit(3);
            }
        }
    }

    if !same(&y, &y_snapshot, 32) {
        std::process::exit(4);
    }

    if sum(&x, 32) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}