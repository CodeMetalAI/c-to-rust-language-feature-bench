static GATE4: i32 = 1;

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

fn same(a: &[i32], b: &[i32], n: usize) -> i32 {
    for i in 0..n {
        if a[i] != b[i] {
            return 0;
        }
    }
    1
}

fn sum(a: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    s
}

fn main() {
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    {
        let p1 = &mut x;
        let q1 = &y;

        if GATE4 != 0 {
            let p2 = p1;
            let q2 = q1;

            copy_restrict(16, &mut p2[8..], &q2[4..]);

            {
                let mut expect = [0i32; 16];
                for i in 0..16 {
                    expect[i] = y_snapshot[4 + i];
                }
                if same(&x[8..], &expect, 16) == 0 {
                    std::process::exit(1);
                }
            }

            if same(&y, &y_snapshot, 32) == 0 {
                std::process::exit(2);
            }
        }

        copy_restrict(8, &mut p1[0..], &q1[24..]);

        {
            let mut expect2 = [0i32; 8];
            for i in 0..8 {
                expect2[i] = y_snapshot[24 + i];
            }
            if same(&x[0..], &expect2, 8) == 0 {
                std::process::exit(3);
            }
        }
    }

    if same(&y, &y_snapshot, 32) == 0 {
        std::process::exit(4);
    }

    if sum(&x, 32) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}