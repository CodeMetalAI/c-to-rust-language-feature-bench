// array_decl_3.rs
const N: usize = 6;
const M: usize = N + 1;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; M]; N]][], p: &[[[i32; M]; N]]) -> bool {
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
        p = p.offset(1);
    }
    false
}

fn check_c(nn: usize, mm: usize, c: [[[i32; N]; N]][[M]], r: [[[i32; N]; N]]) -> bool {
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
    let n = N;
    let m = M;

    let mut a: [[[i32; M]; N]] = [[0; M]; N];
    let mut p: *mut [[i32; M]; N] = a.as_mut_ptr();

    let mut c: [[[i32; N]; N]][M] = [[[0; N]; N]; M];
    let mut r: *mut [[i32; N]; N] = c.as_mut_ptr();

    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < m {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < m {
                    c[i][j][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    p = a.as_mut_ptr();
    if check_a(n, m, &a, p) {
        unsafe {
            GATE = 1;
        }
    }

    r = c.as_mut_ptr();
    if check_c(n, m, c, r) {
        unsafe {
            GATE = 2;
        }
    }
}

fn main() {
    let n = N;
    let m = M;

    unsafe {
        GATE = 0;
    }
    fcompat();

    if GATE != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}