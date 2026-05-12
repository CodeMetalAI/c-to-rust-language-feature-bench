use std::sync::atomic::{AtomicI32, Ordering};

static N: AtomicI32 = AtomicI32::new(0);
static M: AtomicI32 = AtomicI32::new(0);
static GATE: AtomicI32 = AtomicI32::new(0);

fn check_a(nn: i32, mm: i32, a: &[Vec<Vec<i32>>], p: &[Vec<Vec<i32>>]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i as usize][j as usize][k as usize] != p[i as usize][j as usize][k as usize] + i {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn check_c(nn: i32, mm: i32, c: &[Vec<Vec<Vec<i32>>>], r: &[Vec<Vec<Vec<i32>>>]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i as usize][j as usize][u as usize][v as usize] != r[i as usize][j as usize][u as usize][v as usize] {
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
    let nn = N.load(Ordering::SeqCst);
    let mm = M.load(Ordering::SeqCst);

    let mut a = vec![vec![vec![0; mm as usize]; 6]; nn as usize];
    let mut c = vec![vec![vec![vec![0; mm as usize]; 6]; nn as usize]; nn as usize];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i as usize][j as usize][k as usize] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i as usize][j as usize][u as usize][v as usize] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let p = &a;
    if check_a(nn, mm, &a, p) != 0 {
        GATE.store(1, Ordering::SeqCst);
    }

    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        GATE.store(2, Ordering::SeqCst);
    }
}

fn main() {
    N.store(6, Ordering::SeqCst);
    M.store(N.load(Ordering::SeqCst) + 1, Ordering::SeqCst);

    GATE.store(0, Ordering::SeqCst);
    fcompat();

    if GATE.load(Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }
}