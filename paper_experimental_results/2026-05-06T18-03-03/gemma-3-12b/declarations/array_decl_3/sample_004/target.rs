fn main() {
    let mut n = 6;
    let mut m = n + 1;

    let mut gate = 0;

    let nn = n;
    let mm = m;

    let mut a = vec![vec![vec![0; mm]; 6]; nn];
    let mut p = &a[0][0][0];

    let mut c = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];
    let mut r = &c[0][0][0][0];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i][j][u][v] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a, p) != 0 {
        gate = 1;
    }

    if check_c(nn, mm, &c, r) != 0 {
        gate = 2;
    }

    if gate != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<i32>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}