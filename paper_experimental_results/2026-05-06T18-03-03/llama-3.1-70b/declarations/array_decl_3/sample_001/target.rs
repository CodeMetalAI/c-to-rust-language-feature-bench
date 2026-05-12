fn check_a(nn: usize, mm: usize, a: &[[[i32; mm]; 6]; nn], p: &[[[i32; mm]; 6]]) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k]!= p[i][j][k] + i as i32 {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; mm]; 6]; nn]; nn], r: &[[[[i32; mm]; 6]; nn]]) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v]!= r[i][j][u][v] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat(n: usize, m: usize) -> i32 {
    let mut a = vec![vec![vec![0; m]; 6]; n];
    let mut c = vec![vec![vec![vec![0; m]; 6]; n]; n];

    for i in 0..n {
        for j in 0..6 {
            for k in 0..m {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1) as i32;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for u in 0..6 {
                for v in 0..m {
                    c[i][j][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1) as i32;
                }
            }
        }
    }

    let p = &a;
    let r = &c;

    if check_a(n, m, &a, p)!= 0 {
        return 1;
    }

    if check_c(n, m, &c, r)!= 0 {
        return 2;
    }

    0
}

fn main() {
    let n = 6;
    let m = n + 1;

    let gate = fcompat(n, m);

    if gate!= 0 {
        std::process::exit(1);
    }
}