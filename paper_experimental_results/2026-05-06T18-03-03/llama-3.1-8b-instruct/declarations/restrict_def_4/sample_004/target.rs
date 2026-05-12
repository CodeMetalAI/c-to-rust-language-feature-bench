use std::mem;

fn copy_restrict(p: &mut [i32], q: &mut [i32]) {
    let n = p.len().min(q.len());
    for (p, q) in p.iter_mut().take(n).zip(q.iter_mut().take(n)) {
        *p = *q;
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (p, i) in p.iter_mut().zip(0..) {
        *p = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a == b)
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

    y_snapshot.copy_from_slice(&y);

    {
        let mut p1 = &mut x;
        let mut q1 = &mut y;

        if 1 == 1 {
            let mut p2 = p1;
            let mut q2 = q1;

            copy_restrict(&mut p2[8..16], &mut q2[4..10]);

            let expect: [i32; 16] = y_snapshot[4..10].try_into().unwrap();
            if!same(&x[8..16], &expect) {
                panic!("copy_restrict failed");
            }

            if!same(&y, &y_snapshot) {
                panic!("y not same as y_snapshot");
            }
        }

        copy_restrict(&mut p1[..8], &mut q1[24..32]);

        let expect2: [i32; 8] = y_snapshot[24..32].try_into().unwrap();
        if!same(&x[..8], &expect2) {
            panic!("copy_restrict failed");
        }
    }

    if!same(&y, &y_snapshot) {
        panic!("y not same as y_snapshot");
    }

    if sum(&x) == 0 {
        panic!("sum of x is 0");
    }
}