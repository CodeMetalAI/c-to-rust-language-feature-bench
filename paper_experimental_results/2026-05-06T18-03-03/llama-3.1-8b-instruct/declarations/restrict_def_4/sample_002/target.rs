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
    for i in 0..n {
        if a[i]!= b[i] {
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
        for (i, y_snapshot) in y_snapshot.iter_mut().enumerate() {
            *y_snapshot = y[i];
        }
    }

    {
        let p1 = &mut x;
        let q1 = &mut y;

        if GATE4 {
            let p2 = &mut x[8..];
            let q2 = &mut y[4..];

            copy_restrict(16, p2.as_mut_ptr(), q2.as_mut_ptr());

            let expect: [i32; 16] = [
                y_snapshot[4],
                y_snapshot[5],
                y_snapshot[6],
                y_snapshot[7],
                y_snapshot[8],
                y_snapshot[9],
                y_snapshot[10],
                y_snapshot[11],
                y_snapshot[12],
                y_snapshot[13],
                y_snapshot[14],
                y_snapshot[15],
                y_snapshot[16],
                y_snapshot[17],
                y_snapshot[18],
                y_snapshot[19],
            ];

            if!same(&x[8..16], &expect, 16) {
                panic!("same failed");
            }

            if!same(&y[..], &y_snapshot[..], 32) {
                panic!("same failed");
            }
        }

        copy_restrict(8, p1.as_mut_ptr(), q1.as_mut_ptr().offset(24));

        let expect2: [i32; 8] = [
            y_snapshot[24],
            y_snapshot[25],
            y_snapshot[26],
            y_snapshot[27],
            y_snapshot[28],
            y_snapshot[29],
            y_snapshot[30],
            y_snapshot[31],
        ];

        if!same(&x[..8], &expect2, 8) {
            panic!("same failed");
        }
    }

    if!same(&y[..], &y_snapshot[..], 32) {
        panic!("same failed");
    }

    if sum(&x[..]) == 0 {
        panic!("sum failed");
    }
}