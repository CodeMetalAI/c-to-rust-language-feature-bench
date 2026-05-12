static mut N: i32 = 0;
static mut M: i32 = 0;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[[[i32; 6]; 6]; 7]) -> bool {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != a[0][j][k] + i as i32 {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &[[[[i32; 6]; 6]; 6]; 7]) -> bool {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != c[i][j][u][v] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    unsafe {
        let nn = N as usize;
        let mm = M as usize;

        let mut a = [[[0; 7]; 6]; 7];
        let mut c = [[[[0; 7]; 6]; 6]; 7];

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
                        c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000
                            + (u as i32 + 1) * 100
                            + (v as i32 + 1);
                    }
                }
            }
        }

        if check_a(nn, mm, &a) {
            GATE = 1;
        }

        if check_c(nn, mm, &c) {
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