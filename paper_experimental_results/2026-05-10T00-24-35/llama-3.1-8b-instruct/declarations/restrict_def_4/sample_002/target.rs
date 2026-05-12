use std::mem;

const GATE4: bool = true;

fn copy_restrict(p: &mut [i32], q: &mut [i32]) {
    let len = p.len().min(q.len());
    for (p, q) in p.iter_mut().zip(q.iter_mut()).take(len) {
        *p = *q;
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (p, i) in p.iter_mut().zip(0..) {
        *p = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
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
        let mut i = 0;
        while i < 32 {
            y_snapshot[i] = y[i];
            i += 1;
        }
    }

    {
        let mut p1 = x.as_mut_slice();
        let mut q1 = y.as_mut_slice();

        if GATE4 {
            let mut p2 = p1.as_mut_slice();
            let mut q2 = q1.as_mut_slice();

            copy_restrict(&mut p2[8..16], &mut q2[4..12]);

            {
                let mut expect = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if!same(&x[8..16], &expect[..]) {
                    panic!("copy_restrict failed");
                }
            }

            if!same(&y[..], &y_snapshot[..]) {
                panic!("same failed");
            }
        }

        copy_restrict(&mut p1[..8], &mut q1[24..32]);

        {
            let mut expect2 = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if!same(&x[..8], &expect2[..]) {
                panic!("copy_restrict failed");
            }
        }
    }

    if!same(&y[..], &y_snapshot[..]) {
        panic!("same failed");
    }

    if sum(&x[..])!= 0 {
        panic!("sum failed");
    }
}