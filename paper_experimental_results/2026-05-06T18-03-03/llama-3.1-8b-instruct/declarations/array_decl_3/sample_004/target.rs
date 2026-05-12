// array_decl_3.rs
extern crate core;

const N: usize = 6;
const M: usize = N + 1;

static mut GATE: u8 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[u32; mm]; nn]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != (j + 1) * 100 + (k + 1) + i * 10000 {
                    return true;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &[[[[u32; mm]; nn]; nn]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != (v + 1) * 100 + (u + 1) * 100 + (j + 1) * 10000 + i * 1000000 {
                        return true;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    false
}

fn fcompat() {
    let n = N;
    let m = M;

    let mut a: [[[u32; M]; N]] = [[0; M]; N];
    let mut p: &[[u32; M]; N] = &a;

    let mut c: [[[[u32; M]; N]; N]] = [[[0; M]; N]; N];
    let mut r: &[[[u32; M]; N]; N] = &c;

    for i in 0..n {
        for j in 0..6 {
            for k in 0..m {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for u in 0..6 {
                for v in 0..m {
                    c[i][j][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    p = &a;
    if check_a(n, m, &a, p) {
        unsafe {
            GATE = 1;
        }
    }

    r = &c;
    if check_c(n, m, &c, r) {
        unsafe {
            GATE = 2;
        }
    }
}

fn main() {
    let n = N;
    let m = N + 1;

    unsafe {
        GATE = 0;
    }
    fcompat();

    if GATE != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}