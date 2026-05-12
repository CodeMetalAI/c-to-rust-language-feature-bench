use std::sync::atomic::{AtomicI32, Ordering};

static N: AtomicI32 = AtomicI32::new(0);
static M: AtomicI32 = AtomicI32::new(0);
static GATE: AtomicI32 = AtomicI32::new(0);

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0usize;
    let mut p_index = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                if a[i][j][k] != a[p_index][j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_index += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < nn {
            let mut u = 0usize;
            while u < 6 {
                let mut v = 0usize;
                while v < mm {
                    if c[i][j][u][v] != c[i][j][u][v] {
                        return 1;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn fcompat() {
    let nn = N.load(Ordering::SeqCst) as usize;
    let mm = M.load(Ordering::SeqCst) as usize;

    let mut a = vec![vec![vec![0i32; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0i32; mm]; 6]; nn]; nn];

    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0usize;
        while j < nn {
            let mut u = 0usize;
            while u < 6 {
                let mut v = 0usize;
                while v < mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000
                        + (j as i32 + 1) * 10000
                        + (u as i32 + 1) * 100
                        + (v as i32 + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a) != 0 {
        GATE.store(1, Ordering::SeqCst);
    }

    if check_c(nn, mm, &c) != 0 {
        GATE.store(2, Ordering::SeqCst);
    }
}

fn main() {
    N.store(6, Ordering::SeqCst);
    let n = N.load(Ordering::SeqCst);
    M.store(n + 1, Ordering::SeqCst);

    GATE.store(0, Ordering::SeqCst);
    fcompat();

    if GATE.load(Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }
}