static mut GATE4: i32 = 1;

fn copy_restrict(n: i32, p: &mut [i32], q: &[i32]) {
    let mut count = n;
    let mut i = 0;
    while count > 0 {
        p[i] = q[i];
        i += 1;
        count -= 1;
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        p[i] = base + (i as i32) * 5 + 1;
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
        let gate4 = unsafe { GATE4 };

        if gate4 != 0 {
            copy_restrict(16, &mut x[8..24], &y[4..20]);

            {
                let mut expect = [0i32; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..24], &expect, 16) {
                    std::process::exit(1);
                }
            }

            if !same(&y, &y_snapshot, 32) {
                std::process::exit(2);
            }
        }

        copy_restrict(8, &mut x[0..8], &y[24..32]);

        {
            let mut expect2 = [0i32; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[0..8], &expect2, 8) {
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