fn check_a(nn: usize, mm: usize, a: &[[[i32; mm]; 6]; nn], p: &[[[i32; mm]; 6]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[j][k] + i as i32 {
                    return true;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p = &p[1..];
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; mm]; 6]; mm]; nn]; nn], r: &[[[[i32; 6]; mm]; nn]]) -> bool {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return true;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    false
}

fn fcompat() {
    let nn = 6;
    let mm = nn + 1;

    let mut a = [[[0; mm]; 6]; nn];
    let mut p = &a;

    let mut c = [[[[0; 6]; mm]; nn]; nn];
    let mut r = &c;

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
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
                        (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a, p) {
        std::process::exit(1);
    }

    if check_c(nn, mm, &c, r) {
        std::process::exit(2);
    }
}

fn main() {
    fcompat();
}