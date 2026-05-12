fn main() {
    let n: usize = 6;
    let m: usize = n + 1;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; m]; 6]; n];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; m]; 6]; n]; n];

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
                    c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
            }
        }
    }

    let mut gate: i32 = 0;

    let mut p = &a;
    if check_a(&a, &p, n, m)!= 0 {
        gate = 1;
    }

    let mut r = &c;
    if check_c(&c, &r, n, m)!= 0 {
        gate = 2;
    }

    if gate!= 0 {
        std::process::exit(1);
    }
}

fn check_a(a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>, nn: usize, mm: usize) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k]!= p[i][j][k] + (i as i32) {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>, nn: usize, mm: usize) -> i32 {
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