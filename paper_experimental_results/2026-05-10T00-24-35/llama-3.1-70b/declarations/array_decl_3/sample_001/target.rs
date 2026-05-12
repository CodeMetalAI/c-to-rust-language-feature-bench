fn main() {
    let n = 6;
    let m = n + 1;

    let mut gate = 0;

    let mut a = vec![vec![vec![0; m]; 6]; n];
    let mut c = vec![vec![vec![vec![0; m]; 6]; n]; n];

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

    fn check_a(n: usize, m: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> bool {
        for i in 0..n {
            for j in 0..6 {
                for k in 0..m {
                    if a[i][j][k] != p[i][j][k] + i as i32 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn check_c(n: usize, m: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> bool {
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

    if check_a(n, m, &a, &a) || check_c(n, m, &c, &c) {
        gate = 1;
    }

    if gate != 0 {
        std::process::exit(1);
    }
}