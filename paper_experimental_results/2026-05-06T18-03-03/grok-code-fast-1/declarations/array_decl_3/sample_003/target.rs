fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, mut p_idx: usize) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != a[p_idx][j][k] + (i as i32) {
                    return 1;
                }
            }
        }
        p_idx += 1;
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
    let mut gate = 0;
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = ((i as i32) + 1) * 10000 + ((j as i32) + 1) * 100 + ((k as i32) + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = ((i as i32) + 1) * 1000000 + ((j as i32) + 1) * 10000 + ((u as i32) + 1) * 100 + ((v as i32) + 1);
                }
            }
        }
    }

    let p_idx = 0;
    if check_a(nn, mm, &a, p_idx) != 0 {
        gate = 1;
    }

    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        gate = 2;
    }

    gate
}

fn main() {
    let n: usize = 6;
    let m: usize = n + 1;
    let gate = fcompat(n, m);
    if gate != 0 {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}