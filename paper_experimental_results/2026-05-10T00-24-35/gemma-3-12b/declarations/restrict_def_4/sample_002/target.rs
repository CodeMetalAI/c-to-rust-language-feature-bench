fn main() {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    {
        let mut p1 = &mut x[0];
        let mut q1 = &mut y[0];

        if gate4() {
            let mut p2 = p1;
            let mut q2 = q1;

            copy_restrict(p2, q2, 16);

            let expect = [0; 16];
            let mut i = 0;
            while i < 16 {
                let index = 4 + i;
                expect[i] = y_snapshot[index];
                i += 1;
            }

            if !same(&x[8], &expect, 16) {
                return 1;
            }

            if !same(&y, &y_snapshot, 32) {
                return 2;
            }
        }

        copy_restrict(p1, q1, 8);

        let expect2 = [0; 8];
        let mut i = 0;
        while i < 8 {
            let index = 24 + i;
            expect2[i] = y_snapshot[index];
            i += 1;
        }

        if !same(&x[0], &expect2, 8) {
            return 3;
        }
    }

    if !same(&y, &y_snapshot, 32) {
        return 4;
    }

    if sum(&x, 32) == 0 {
        return 5;
    }

    0
}

volatile static mut GATE4: i32 = 1;

fn gate4() -> bool {
    unsafe { GATE4 }
}

fn copy_restrict(p: *mut i32, q: *mut i32, n: usize) {
    unsafe {
        let mut p_mut = p;
        let mut q_mut = q;
        for _ in 0..n {
            *p_mut = *q_mut;
            p_mut = p_mut.add(1);
            q_mut = q_mut.add(1);
        }
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn sum(a: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    s
}