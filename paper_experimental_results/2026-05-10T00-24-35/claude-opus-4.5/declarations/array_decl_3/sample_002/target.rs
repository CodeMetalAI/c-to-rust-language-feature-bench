fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[0][j][k] + i as i32 {
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

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
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

fn fcompat(n: usize, m: usize) -> i32 {
    let nn = n;
    let mm = m;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    let mut gate = 0;

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
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
                        ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    // p = a (p points to the beginning of a)
    // check_a checks if a[i][j][k] == (*p)[j][k] + i where p advances with i
    // Since p starts at a[0] and p += 1 advances to a[1], etc.,
    // this checks if a[i][j][k] == a[0][j][k] + i
    if check_a(nn, mm, &a, &a) != 0 {
        gate = 1;
    }

    // r = c (r points to the beginning of c)
    // check_c checks if c[i][j][u][v] == r[i][j][u][v]
    // Since r = c, this should always be true
    if check_c(nn, mm, &c, &c) != 0 {
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
    }

    std::process::exit(0);
}