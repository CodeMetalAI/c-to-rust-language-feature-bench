// array_decl_3.rs
extern "C" {
    static mut n: i32;
    static mut m: i32;
}

static mut gate: i32 = 0;

fn check_a(nn: i32, mm: i32, a: &mut [[i32; 6]; nn as usize], p: &mut [[i32; mm as usize]; nn as usize]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i as usize][j][k] != (p[j][k] + i as i32) as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p = &mut p[i as usize];
    }
    0
}

fn check_c(nn: i32, mm: i32, c: &mut [[[i32; 6]; nn as usize]; nn as usize], r: &mut [[[i32; mm as usize]; 6]; nn as usize]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i as usize][j as usize][u][v] != (r[i as usize][j as usize][u][v]) {
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
    let nn = n;
    let mm = m;

    let mut a = [[[0; mm as usize]; 6]; nn as usize];
    let mut p = &mut a[0];

    let mut c = [[[0; mm as usize]; 6]; nn as usize];
    let mut r = &mut c[0];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i as usize][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
        p = &mut a[i];
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i as usize][j as usize][u][v] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
        r = &mut c[i];
    }

    if check_a(nn, mm, &mut a, &mut p) != 0 {
        gate = 1;
    }

    if check_c(nn, mm, &mut c, &mut r) != 0 {
        gate = 2;
    }
}

fn main() {
    n = 6;
    m = n + 1;

    gate = 0;
    fcompat();

    if gate != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}