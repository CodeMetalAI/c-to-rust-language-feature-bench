// array_decl_3.rs

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; mm]], p: &mut [[[i32; 6]; mm]]) -> i32 {
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
        p = &mut p[1..];
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; mm]; nn]; nn], r: &mut [[[i32; 6]; mm]; nn]) -> i32 {
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

fn fcompat(n: usize, m: usize) {
    let nn = n;
    let mm = m;

    let mut a = vec![vec![vec![0; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = (i + 1) as i32 * 10000 + (j + 1) as i32 * 100 + (k + 1) as i32;
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
                        (i + 1) as i32 * 1000000 + (j + 1) as i32 * 10000 + (u + 1) as i32 * 100 + (v + 1) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let mut p = &mut a;
    if check_a(nn, mm, &a, p) != 0 {
        unsafe { GATE = 1; }
    }

    let mut r = &mut c;
    if check_c(nn, mm, &c, r) != 0 {
        unsafe { GATE = 2; }
    }
}

fn main() {
    let n = 6;
    let m = n + 1;

    unsafe { GATE = 0; }
    fcompat(n, m);

    if unsafe { GATE != 0 } {
        std::process::exit(1);
    }

    std::process::exit(0);
}