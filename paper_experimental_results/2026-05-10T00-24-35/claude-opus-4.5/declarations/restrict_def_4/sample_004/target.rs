use std::sync::atomic::{AtomicI32, Ordering};

static GATE4: AtomicI32 = AtomicI32::new(1);

fn copy_slices(dest: &mut [i32], src: &[i32], n: usize) {
    let mut i = 0;
    let mut count = n;
    while count > 0 {
        dest[i] = src[i];
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
        if GATE4.load(Ordering::SeqCst) != 0 {
            // copy_restrict(16, p2 + 8, q2 + 4) means copy 16 elements from y[4..] to x[8..]
            for i in 0..16 {
                x[8 + i] = y[4 + i];
            }

            {
                let mut expect: [i32; 16] = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..], &expect, 16) {
                    std::process::exit(1);
                }
            }

            if !same(&y, &y_snapshot, 32) {
                std::process::exit(2);
            }
        }

        // copy_restrict(8, p1 + 0, q1 + 24) means copy 8 elements from y[24..] to x[0..]
        for i in 0..8 {
            x[i] = y[24 + i];
        }

        {
            let mut expect2: [i32; 8] = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[0..], &expect2, 8) {
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