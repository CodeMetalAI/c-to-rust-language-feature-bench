static mut N: i32 = 0;
static mut M: i32 = 0;
static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[Vec<Vec<i32>>], p_offset: usize) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != a[p_offset][j][k] + (i as i32) {
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

fn check_c(nn: usize, mm: usize, c: &[Vec<Vec<Vec<i32>>>], r: &[Vec<Vec<Vec<i32>>>]) -> i32 {
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

fn fcompat() {
    let nn = unsafe { N as usize };
    let mm = unsafe { M as usize };

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

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
                    c[i][j][u][v] = ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a, 0) != 0 {
        unsafe { GATE = 1; }
    }

    if check_c(nn, mm, &c, &c) != 0 {
        unsafe { GATE = 2; }
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;
        GATE = 0;
    }

    fcompat();

    let exit_code = unsafe { if GATE != 0 { 1 } else { 0 } };
    std::process::exit(exit_code);
}