use std::sync::atomic::{AtomicI32, Ordering};

static GATE4: AtomicI32 = AtomicI32::new(1);

fn copy_slices(dest: &mut [i32], src: &[i32]) {
    let mut n = dest.len().min(src.len());
    let mut i = 0;
    while n > 0 {
        dest[i] = src[i];
        i += 1;
        n -= 1;
    }
}

fn fill(p: &mut [i32], base: i32) {
    let n = p.len();
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
        if GATE4.load(Ordering::SeqCst) != 0 {
            // copy_restrict(16, p2 + 8, q2 + 4) means copy 16 elements from y[4..20] to x[8..24]
            let src: Vec<i32> = y[4..20].to_vec();
            copy_slices(&mut x[8..24], &src);

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

        // copy_restrict(8, p1 + 0, q1 + 24) means copy 8 elements from y[24..32] to x[0..8]
        let src: Vec<i32> = y[24..32].to_vec();
        copy_slices(&mut x[0..8], &src);

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