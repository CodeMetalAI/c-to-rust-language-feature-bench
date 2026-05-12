// restrict_def_4.rs
static mut GATE4: i32 = 1;

fn copy_restrict(n: usize, p: &mut i32, q: &mut i32) {
    while n > 0 {
        *p = *q;
        p = p.offset(1);
        q = q.offset(1);
        n -= 1;
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, x) in p.iter_mut().enumerate() {
        *x = base + i as i32 * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    for (i, x) in a.iter().zip(b.iter()).take(n) {
        if *x != *i {
            return false;
        }
    }
    true
}

fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn main() {
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    {
        for (x, y) in y_snapshot.iter_mut().zip(&y) {
            *x = *y;
        }
    }

    {
        let mut p1 = &mut x;
        let mut q1 = &mut y;

        if GATE4 {
            let mut p2 = p1.offset(8);
            let mut q2 = q1.offset(4);

            copy_restrict(16, p2, q2);

            {
                let mut expect: [i32; 16] = [0; 16];
                for (expect, y) in expect.iter_mut().zip(&y_snapshot[4..20]) {
                    *expect = *y;
                }
                if !same(&x[8..24], &expect, 16) {
                    panic!("same failed");
                }
            }

            if !same(&y, &y_snapshot, 32) {
                panic!("same failed");
            }
        }

        copy_restrict(8, p1, q1.offset(24));

        {
            let mut expect2: [i32; 8] = [0; 8];
            for (expect2, y) in expect2.iter_mut().zip(&y_snapshot[24..32]) {
                *expect2 = *y;
            }
            if !same(&x[0..8], &expect2, 8) {
                panic!("same failed");
            }
        }
    }

    if !same(&y, &y_snapshot, 32) {
        panic!("same failed");
    }

    if sum(&x) == 0 {
        panic!("sum failed");
    }
}