static mut N: i32 = 0;
static mut M: i32 = 0;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; 6]; 6], p: &mut [[[i32; 6]; 6]; 6]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[i][j][k] + i as i32 {
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

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; 6]; 6]; 6], r: &[[[[i32; 6]; 6]; 6]; 6]) -> i32 {
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
    unsafe {
        let nn = N as usize;
        let mm = M as usize;

        let mut a = [[[0; 6]; 6]; 6];
        let mut c = [[[[0; 6]; 6]; 6]; 6];

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
                        c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                        v += 1;
                    }
                    u += 1;
                }
                j += 1;
            }
            i += 1;
        }

        let mut p = a;
        if check_a(nn, mm, &a, &mut p) != 0 {
            GATE = 1;
        }

        let r = c;
        if check_c(nn, mm, &c, &r) != 0 {
            GATE = 2;
        }
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;

        GATE = 0;
        fcompat();

        if GATE != 0 {
            std::process::exit(1);
        }
    }
}