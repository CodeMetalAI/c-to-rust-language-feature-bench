// array_decl_3.rs
const N: usize = 6;
const M: usize = N + 1;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; M]; N]]) -> bool {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != (j as i32 + 1) * 100 + (k as i32 + 1) + (i + 1) * 10000 {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; M]; N]; N]]) -> bool {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != (u as i32 + 1) * 100 + (v as i32 + 1) + (j as i32 + 1) * 10000 + (i + 1) * 1000000 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    let mut a: [[[i32; M]; N]] = [[0; N]; N];
    let mut c: [[[[i32; M]; N]; N]] = [[[0; N]; N]; N];

    for i in 0..N {
        for j in 0..6 {
            for k in 0..M {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..N {
        for j in 0..N {
            for u in 0..6 {
                for v in 0..M {
                    c[i][j][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    let p = &a;
    if check_a(N, M, &a) {
        GATE = 1;
    }

    let r = &c;
    if check_c(N, M, &c) {
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