// array_decl_3.rs

static mut N: i32 = 0;
static mut M: i32 = 0;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != p[i][j][k] + i as i32 {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat() {
    let nn = unsafe { N as usize };
    let mm = unsafe { M as usize };

    let mut a = vec![vec![vec![0; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
            }
        }
    }

    if check_a(nn, mm, &a, &a) != 0 {
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
        fcompat();

        if GATE != 0 {
            std::process::exit(1);
        }
    }
}