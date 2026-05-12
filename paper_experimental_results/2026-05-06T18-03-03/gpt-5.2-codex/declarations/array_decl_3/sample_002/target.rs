use std::process::exit;

fn idx_a(i: usize, j: usize, k: usize, mm: usize) -> usize {
    (i * 6 + j) * mm + k
}

fn idx_c(i: usize, j: usize, u: usize, v: usize, nn: usize, mm: usize) -> usize {
    (((i * nn + j) * 6 + u) * mm) + v
}

fn check_a(nn: usize, mm: usize, a: &Vec<i32>) -> i32 {
    let mut p_i: usize = 0;
    let mut i: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < 6 {
            let mut k: usize = 0;
            while k < mm {
                let a_val = a[idx_a(i, j, k, mm)];
                let p_val = a[idx_a(p_i, j, k, mm)];
                if a_val != p_val + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_i += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<i32>) -> i32 {
    let mut i: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < nn {
            let mut u: usize = 0;
            while u < 6 {
                let mut v: usize = 0;
                while v < mm {
                    let val = c[idx_c(i, j, u, v, nn, mm)];
                    let r_val = c[idx_c(i, j, u, v, nn, mm)];
                    if val != r_val {
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

fn fcompat(nn: usize, mm: usize) -> i32 {
    let mut a = vec![0i32; nn * 6 * mm];
    let mut c = vec![0i32; nn * nn * 6 * mm];

    let mut i: usize = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < 6 {
            let mut k: usize = 0;
            while k < mm {
                a[idx_a(i, j, k, mm)] =
                    (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j: usize = 0;
        while j < nn {
            let mut u: usize = 0;
            while u < 6 {
                let mut v: usize = 0;
                while v < mm {
                    c[idx_c(i, j, u, v, nn, mm)] = (i as i32 + 1) * 1_000_000
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

    let mut gate: i32 = 0;

    if check_a(nn, mm, &a) != 0 {
        gate = 1;
    }

    if check_c(nn, mm, &c) != 0 {
        gate = 2;
    }

    gate
}

fn main() {
    let n: usize = 6;
    let m: usize = n + 1;

    let gate = fcompat(n, m);

    if gate != 0 {
        exit(1);
    }
    exit(0);
}