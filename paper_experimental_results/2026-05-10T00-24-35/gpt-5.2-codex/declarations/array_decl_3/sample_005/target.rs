fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0usize;
    let mut p_idx = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                if a[i][j][k] != a[p_idx][j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_idx += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < nn {
            let mut u = 0usize;
            while u < 6 {
                let mut v = 0usize;
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

fn fcompat(n: i32, m: i32, gate: &mut i32) {
    let nn = n as usize;
    let mm = m as usize;

    let mut a = vec![vec![vec![0i32; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0i32; mm]; 6]; nn]; nn];

    let mut i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < 6 {
            let mut k = 0usize;
            while k < mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0usize;
    while i < nn {
        let mut j = 0usize;
        while j < nn {
            let mut u = 0usize;
            while u < 6 {
                let mut v = 0usize;
                while v < mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000
                        + (j as i32 + 1) * 10000
                        + (u as i32 + 1) * 100
                        + (v as i32 + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a) != 0 {
        *gate = 1;
    }

    if check_c(nn, mm, &c, &c) != 0 {
        *gate = 2;
    }
}

fn main() {
    let n: i32 = 6;
    let m: i32 = n + 1;

    let mut gate: i32 = 0;
    fcompat(n, m, &mut gate);

    if gate != 0 {
        std::process::exit(1);
    }
}