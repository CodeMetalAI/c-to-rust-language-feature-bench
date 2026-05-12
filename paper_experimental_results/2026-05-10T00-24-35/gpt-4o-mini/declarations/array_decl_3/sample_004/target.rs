static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; 6]; 6], p: &[[[i32; 6]; 6]; 6]) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != p[j][k] + i as i32 {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; 6]; 6]; 6], r: &[[[[i32; 6]; 6]; 6]) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat(n: usize, m: usize) {
    let mut a: [[[i32; 6]; 6]; 6] = [[[0; 6]; 6]; 6];
    let mut c: [[[[i32; 6]; 6]; 6]; 6] = [[[[0; 6]; 6]; 6]; 6];

    for i in 0..n {
        for j in 0..6 {
            for k in 0..m {
                a[i][j][k] = (i + 1) as i32 * 10000 + (j + 1) as i32 * 100 + (k + 1) as i32;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for u in 0..6 {
                for v in 0..m {
                    c[i][j][u][v] = (i + 1) as i32 * 1000000 + (j + 1) as i32 * 10000 + (u + 1) as i32 * 100 + (v + 1) as i32;
                }
            }
        }
    }

    let p = &a;
    if check_a(n, m, &a, p) != 0 {
        unsafe { GATE = 1; }
    }

    let r = &c;
    if check_c(n, m, &c, r) != 0 {
        unsafe { GATE = 2; }
    }
}

fn main() {
    let n: usize = 6;
    let m: usize = n + 1;

    unsafe { GATE = 0; }
    fcompat(n, m);

    if unsafe { GATE } != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}