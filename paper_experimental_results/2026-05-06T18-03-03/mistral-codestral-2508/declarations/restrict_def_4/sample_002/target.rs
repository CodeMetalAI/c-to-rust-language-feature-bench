fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn fill(p: &mut [i32], base: i32) {
    let mut i = 0;
    while i < p.len() {
        p[i] = base + i as i32 * 5 + 1;
        i += 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn sum(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < a.len() {
        s += a[i];
        i += 1;
    }
    s
}

fn main() {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    {
        let mut i = 0;
        while i < 32 {
            y_snapshot[i] = y[i];
            i += 1;
        }
    }

    {
        let p1 = &mut x[..];
        let q1 = &y[..];

        if true {
            let p2 = &mut p1[8..];
            let q2 = &q1[4..];

            copy_restrict(16, p2, q2);

            {
                let mut expect = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..24], &expect) {
                    std::process::exit(1);
                }
            }

            if !same(&y, &y_snapshot) {
                std::process::exit(2);
            }
        }

        copy_restrict(8, &mut p1[0..8], &q1[24..]);

        {
            let mut expect2 = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[0..8], &expect2) {
                std::process::exit(3);
            }
        }
    }

    if !same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}