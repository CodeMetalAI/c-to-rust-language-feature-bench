fn copy_restrict(n: usize, dst: &mut [i32], src: &[i32]) {
    for i in 0..n {
        dst[i] = src[i];
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

fn main() -> i32 {
    const GATE4: bool = true;
    let mut x = [0i32; 32];
    let mut y = [0i32; 32];
    let mut y_snapshot = [0i32; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    {
        let mut p1 = &mut x[..];
        let mut q1 = &y[..];

        if GATE4 {
            let p2 = &mut p1[8..];
            let q2 = &q1[4..];

            copy_restrict(16, p2, q2);

            {
                let mut expect = [0i32; 16];
                for i in 0..16 {
                    expect[i] = y_snapshot[4 + i];
                }
                if same(&x[8..], &expect, 16) != 1 {
                    return 1;
                }
            }

            if same(&y, &y_snapshot, 32) != 1 {
                return 2;
            }
        }

        copy_restrict(8, &mut p1[0..], &q1[24..]);

        {
            let mut expect2 = [0i32; 8];
            for i in 0..8 {
                expect2[i] = y_snapshot[24 + i];
            }
            if same(&x[0..], &expect2, 8) != 1 {
                return 3;
            }
        }
    }

    if same(&y, &y_snapshot, 32) != 1 {
        return 4;
    }

    if sum(&x, 32) == 0 {
        return 5;
    }

    0
}