use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static N: AtomicI32 = AtomicI32::new(0);
static M: AtomicI32 = AtomicI32::new(0);
static GATE: AtomicI32 = AtomicI32::new(0);

fn idx_a(i: usize, j: usize, k: usize, mm: usize) -> usize {
    (i * 6 + j) * mm + k
}

fn idx_c(i: usize, j: usize, u: usize, v: usize, nn: usize, mm: usize) -> usize {
    (((i * nn + j) * 6 + u) * mm + v)
}

fn check_a(nn: usize, mm: usize, a: &[i32], mut p_base: usize) -> i32 {
    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                let a_val = a[idx_a(i, j, k, mm)];
                let p_val = a[p_base + idx_a(0, j, k, mm)] + i as i32;
                if a_val != p_val {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_base += 6 * mm;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &[i32]) -> i32 {
    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < nn {
            let mut u = 0usize;
            while u < 6 {
                let mut v = 0usize;
                while v < mm {
                    let idx = idx_c(i, j, u, v, nn, mm);
                    if c[idx] != c[idx] {
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

    let mut a = vec![0i32; nn * 6 * mm];
    let mut c = vec![0i32; nn * nn * 6 * mm];

    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                let val = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
                let idx = idx_a(i, j, k, mm);
                a[idx] = val;
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
                    let val = (i as i32 + 1) * 1_000_000
                        + (j as i32 + 1) * 10000
                        + (u as i32 + 1) * 100
                        + (v as i32 + 1);
                    let idx = idx_c(i, j, u, v, nn, mm);
                    c[idx] = val;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let p_base = 0usize;
    if check_a(nn, mm, &a, p_base) != 0 {
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
        exit(1);
    }

    exit(0);
}