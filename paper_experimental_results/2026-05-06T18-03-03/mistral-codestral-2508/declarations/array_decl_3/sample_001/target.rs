static mut N: i32 = 0;
static mut M: i32 = 0;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; 100]], p: &[[[i32; 6]; 100]]) -> i32 {
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

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 100]; 6]; 100]], r: &[[[[i32; 100]; 6]; 100]]) -> i32 {
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
    let nn = unsafe { N } as usize;
    let mm = unsafe { M } as usize;

    let mut a = [[[0; 6]; 100]; 100];
    let mut c = [[[[0; 100]; 6]; 100]; 100];

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

    let p = &a;
    if check_a(nn, mm, &a, p) != 0 {
        unsafe { GATE = 1 };
    }

    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        unsafe { GATE = 2 };
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;
    }

    unsafe { GATE = 0 };
    fcompat();

    if unsafe { GATE } != 0 {
        std::process::exit(1);
    }
}