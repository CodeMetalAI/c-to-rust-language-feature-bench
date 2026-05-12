fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != p[i][j][k] + i as i32 {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
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

fn fcompat(nn: usize, mm: usize) -> i32 {
    let mut a = vec![vec![vec![0; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i + 1) as i32 * 10000 + (j + 1) as i32 * 100 + (k + 1) as i32;
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = (i + 1) as i32 * 1000000 + (j + 1) as i32 * 10000 + (u + 1) as i32 * 100 + (v + 1) as i32;
                }
            }
        }
    }

    if check_a(nn, mm, &a, &a) != 0 {
        return 1;
    }

    if check_c(nn, mm, &c, &c) != 0 {
        return 2;
    }

    0
}

fn main() {
    let n = 6;
    let m = n + 1;

    let gate = fcompat(n, m);

    if gate != 0 {
        std::process::exit(1);
    }
}