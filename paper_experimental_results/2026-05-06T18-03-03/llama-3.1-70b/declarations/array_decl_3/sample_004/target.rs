fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; mm]; nn], p: &[[[i32; 6]; mm]]) -> i32 {
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

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; mm]; nn]; nn], r: &[[[i32; 6]; mm]; nn]) -> i32 {
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

fn fcompat() {
    let nn = 6;
    let mm = nn + 1;

    let mut a = [[[0; 6]; mm]; nn];
    let mut c = [[[[0; 6]; mm]; nn]; nn];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
            }
        }
    }

    let p = &a;
    let r = &c;

    if check_a(nn, mm, &a, p) != 0 || check_c(nn, mm, &c, r) != 0 {
        std::process::exit(1);
    }
}

fn main() {
    fcompat();
}