use std::sync::atomic::{AtomicI32, Ordering};

static mut N: i32 = 0;
static mut M: i32 = 0;
static GATE: AtomicI32 = AtomicI32::new(0);

fn check_a(nn: usize, mm: usize, a: &[Vec<Vec<i32>>], p_offset: usize) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                let p_idx = p_offset + i;
                if a[i][j][k] != a[p_idx][j][k] + (i as i32) {
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

fn check_c(nn: usize, mm: usize, c: &[Vec<Vec<Vec<i32>>>], r: &[Vec<Vec<Vec<i32>>>]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
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
    let nn = unsafe { N as usize };
    let mm = unsafe { M as usize };

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
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
                    c[i][j][u][v] =
                        ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let p_offset = 0;
    if check_a(nn, mm, &a, p_offset) != 0 {
        GATE.store(1, Ordering::SeqCst);
    }

    if check_c(nn, mm, &c, &c) != 0 {
        GATE.store(2, Ordering::SeqCst);
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;
    }

    GATE.store(0, Ordering::SeqCst);
    fcompat();

    if GATE.load(Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}