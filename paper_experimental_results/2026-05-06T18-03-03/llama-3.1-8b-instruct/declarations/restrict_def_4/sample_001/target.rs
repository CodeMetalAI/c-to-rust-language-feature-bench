// restrict_def_4.rs
static mut GATE4: i32 = 1;

fn copy_restrict(n: usize, p: &mut [i32], q: &mut [i32]) {
    for (p, q) in p.iter_mut().zip(q.iter_mut()) {
        *p = *q;
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    for (a, b) in a.iter().zip(b.iter()) {
        if *a != *b {
            return false;
        }
    }
    true
}

fn sum(a: &[i32], n: usize) -> i32 {
    a.iter().sum()
}

fn main() {
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    {
        for i in 0..32 {
            y_snapshot[i] = y[i];
        }
    }

    {
        let mut p1 = x.as_mut_ptr();
        let mut q1 = y.as_mut_ptr();

        if GATE4 {
            let mut p2 = p1.offset(8);
            let mut q2 = q1.offset(4);

            copy_restrict(16, p2, q2);

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

            if !same(&x[8..24], &expect, 16) {
                panic!("Test failed");
            }

            if !same(&y[..32], &y_snapshot[..32], 32) {
                panic!("Test failed");
            }
        }

        copy_restrict(8, p1, q1.offset(24));

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

        if !same(&x[..8], &expect2, 8) {
            panic!("Test failed");
        }
    }

    if !same(&y[..32], &y_snapshot[..32], 32) {
        panic!("Test failed");
    }

    if sum(&x[..32], 32) == 0 {
        panic!("Test failed");
    }

    assert_eq!(0, 0);
}