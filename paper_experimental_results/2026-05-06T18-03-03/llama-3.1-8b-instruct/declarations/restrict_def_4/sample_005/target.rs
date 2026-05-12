static mut gate4: i32 = 1;

fn copy_restrict(n: i32, p: &mut i32, q: &mut i32) {
    while n > 0 {
        *p = *q;
        p = p.offset(1);
        q = q.offset(1);
        n -= 1;
    }
}

fn fill(p: &mut i32, n: i32, base: i32) {
    let mut i = 0;
    while i < n {
        p[i] = base + i * 5 + 1;
        i += 1;
    }
}

fn same(a: &[i32], b: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn sum(a: &[i32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += a[i];
        i += 1;
    }
    s
}

fn main() {
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

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
        let mut p1 = &mut x;
        let mut q1 = &mut y;

        if gate4 != 0 {
            let mut p2 = p1.offset(8);
            let mut q2 = q1.offset(4);

            copy_restrict(16, p2, q2);

            {
                let mut expect: [i32; 16] = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..24], &expect[..16]) {
                    panic!("Test failed");
                }
            }

            if !same(&y[..32], &y_snapshot[..32]) {
                panic!("Test failed");
            }
        }

        copy_restrict(8, p1, q1.offset(24));

        {
            let mut expect2: [i32; 8] = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[..8], &expect2[..8]) {
                panic!("Test failed");
            }
        }
    }

    if !same(&y[..32], &y_snapshot[..32]) {
        panic!("Test failed");
    }

    if sum(&x[..32], 32) == 0 {
        panic!("Test failed");
    }
}