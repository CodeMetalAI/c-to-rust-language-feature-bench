// array_decl_3.rs

static mut N: i32 = 0;
static mut M: i32 = 0;
static mut GATE: i32 = 0;

fn check_a(nn: i32, mm: i32, a: &[[[i32; M as usize]; 6]; N as usize], p: &[[[i32; M as usize]; 6]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i as usize][j][k] != (p[j][k] + i) {
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

fn check_c(nn: i32, mm: i32, c: &[[[[i32; M as usize]; 6]; N as usize]; N as usize], r: &[[[[i32; M as usize]; 6]; N as usize]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i as usize][j as usize][u][v] != r[i as usize][j as usize][u][v] {
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
    let nn = N;
    let mm = M;

    let mut a: [[[i32; M as usize]; 6]; N as usize] = [[[(0 as i32); M as usize]; 6]; N as usize];
    let mut p: &[[[i32; M as usize]; 6]] = &a;

    let mut c: [[[i32; 6]; N as usize]; N as usize] = [[[[0 as i32; 6]; N as usize]; N as usize]];
    let mut r: &[[[[i32; 6]; N as usize]; N as usize]] = &c;

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i as usize][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i as usize][j as usize][u][v] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
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
    N = 6;
    M = N + 1;

    GATE = 0;
    fcompat();

    if GATE != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}