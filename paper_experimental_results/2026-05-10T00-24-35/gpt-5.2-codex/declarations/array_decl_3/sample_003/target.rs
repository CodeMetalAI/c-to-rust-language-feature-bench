use std::sync::atomic::{AtomicI32, Ordering};
use std::process;

static N: AtomicI32 = AtomicI32::new(0);
static M: AtomicI32 = AtomicI32::new(0);
static GATE: AtomicI32 = AtomicI32::new(0);

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i: usize = 0;
    let mut p_index: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < 6 {
            let mut k: usize = 0;
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
    let mut i: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < nn {
            let mut u: usize = 0;
            while u < 6 {
                let mut v: usize = 0;
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
    let nn_i = N.load(Ordering::SeqCst);
    let mm_i = M.load(Ordering::SeqCst);
    let nn = nn_i as usize;
    let mm = mm_i as usize;

    let mut a = vec![vec![vec![0i32; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0i32; mm]; 6]; nn]; nn];

    let mut i: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < 6 {
            let mut k: usize = 0;
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
        let mut j: usize = 0;
        while j < nn {
            let mut u: usize = 0;
            while u < 6 {
                let mut v: usize = 0;
                while v < mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1_000_000
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
    let n_val = N.load(Ordering::SeqCst);
    M.store(n_val + 1, Ordering::SeqCst);

    GATE.store(0, Ordering::SeqCst);
    fcompat();

    if GATE.load(Ordering::SeqCst) != 0 {
        process::exit(1);
    }
    process::exit(0);
}