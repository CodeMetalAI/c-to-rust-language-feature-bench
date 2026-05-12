// array_decl_3.rs
static mut N: i32 = 0;
static mut M: i32 = 0;

static gated: i32 = 0;

fn check_a<R>(n: i32, m: i32, a: &[[[i32; M as usize]; 6]; N as usize], p: &[[[i32; M as usize]; 6]]) -> bool {
    for i in 0..n {
        for j in 0..6 {
            for k in 0..m {
                if a[i][j][k] != (p[j][k] + i) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(n: i32, m: i32, c: &[[[[i32; M as usize]; 6]; n as usize]; n as usize], r: &[[[[i32; M as usize]; 6]; n as usize]]) -> bool {
    for i in 0..n {
        for j in 0..n {
            for u in 0..6 {
                for v in 0..m {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    let n = N;
    let m = M;

    let mut a = vec![vec![vec![0; M as usize]; 6]; n as usize];
    let mut p = vec![vec![vec![0; M as usize]; 6]; n as usize];

    let mut c = vec![vec![vec![vec![0; M as usize]; 6]; n as usize]; n as usize];
    let mut r = vec![vec![vec![vec![0; M as usize]; 6]; n as usize]; n as usize];

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
                    c[i][j][u][v] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    p = a.clone();
    if check_a(n, m, a.as_slice(), p.as_slice()) {
        gated = 1;
    }

    r = c.clone();
    if check_c(n, m, c.as_slice(), r.as_slice()) {
        gated = 2;
    }
}

fn main() {
    N = 6;
    M = N + 1;

    gated = 0;
    fcompat();

    if gated != 0 {
        std::process::exit(1);
    }
}