fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; mm]; nn], p: &[[[i32; 6]; mm]]) -> i32 {
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

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; mm]; nn]; nn], r: &[[[[i32; 6]; mm]; nn]; nn]) -> i32 {
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
    let mut a: [[[i32; 6]; m]; n] = [[[0; 6]; m]; n];
    let mut c: [[[[i32; 6]; m]; n]; n] = [[[[0; 6]; m]; n]; n];

    for i in 0..n {
        for j in 0..6 {
            for k in 0..m {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for u in 0..6 {
                for v in 0..m {
                    c[i][j][u][v] =
                        (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
            }
        }
    }

    let gate_a = check_a(n, m, &a, &a);
    let gate_c = check_c(n, m, &c, &c);

    if gate_a!= 0 || gate_c!= 0 {
        1
    } else {
        0
    }
}

fn main() {
    let n = 6;
    let m = n + 1;

    let gate = fcompat(n, m);

    if gate!= 0 {
        std::process::exit(1);
    }
}