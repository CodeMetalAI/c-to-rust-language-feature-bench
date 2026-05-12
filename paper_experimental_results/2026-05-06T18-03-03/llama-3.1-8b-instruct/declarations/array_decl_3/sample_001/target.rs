// array_decl_3.rs
use std::mem;

const N: usize = 6;
const M: usize = N + 1;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; M]; N]; N], p: &[[[i32; M]; N]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[j][k] + i as i32 {
                    return true;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p = &p[1..];
    }
    false
}

fn check_c(nn: usize, mm: usize, c: [[[i32; N]; N]; N], r: [[[i32; N]; N]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
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
    let mut nn = N;
    let mut mm = M;

    let mut a: [[[i32; M]; N]; N] = [[[(0 as i32); M]; N]; N];
    let mut p: &[[[i32; M]; N]] = &a;

    let mut c: [[[i32; N]; N]; N] = [[[0 as i32; N]; N]; N];
    let mut r: &[[[i32; N]; N]] = &c;

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    p = &a;
    if check_a(nn, mm, &a, p) {
        GATE = 1;
    }

    r = &c;
    if check_c(nn, mm, &c, r) {
        GATE = 2;
    }
}

fn main() {
    let n = N;
    let m = n + 1;

    GATE = 0;
    fcompat();

    if GATE != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}