// array_decl_3.rs

fn check_a(nn: usize, mm: usize, a: &[[[i32; mm]; 6]; nn], p: &[[[i32; mm]; 6]]) -> bool {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k]!= p[j][k] + i as i32 {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; mm]; 6]; nn]; nn], r: &[[[[i32; mm]; 6]; nn]]) -> bool {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v]!= r[i][j][u][v] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    let nn = n;
    let mm = m;

    let mut a = vec![[[0; mm]; 6]; nn];
    let mut p = a.as_ptr() as *mut [[i32; mm]; 6];

    let mut c = vec![[[[0; mm]; 6]; nn]; nn];
    let mut r = c.as_ptr() as *mut [[[[i32; mm]; 6]; nn]];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    let p = a.as_ptr() as *mut [[i32; mm]; 6];
    if check_a(nn, mm, &a, p) {
        gate = 1;
    }

    let r = c.as_ptr() as *mut [[[[i32; mm]; 6]; nn]];
    if check_c(nn, mm, &c, r) {
        gate = 2;
    }
}

fn main() {
    n = 6;
    m = n + 1;

    gate = 0;
    fcompat();

    if gate!= 0 {
        std::process::exit(1);
    }
}

static mut n: usize = 0;
static mut m: usize = 0;
static mut gate: usize = 0;