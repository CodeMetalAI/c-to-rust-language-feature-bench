fn main() {
    static mut N: i32 = 0;
    static mut M: i32 = 0;
    static mut GATE: i32 = 0;

    unsafe {
        N = 6;
        M = N + 1;

        GATE = 0;
        fcompat();

        if GATE != 0 {
            std::process::exit(1);
        }
        std::process::exit(0);
    }
}

unsafe fn fcompat() {
    let nn = N as usize;
    let mm = M as usize;

    // Create dynamically sized arrays using vectors
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    // Initialize array a
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
            }
        }
    }

    // Initialize array c
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                }
            }
        }
    }

    // Check array a
    let p = &a;
    if check_a(nn, mm, &a, p) != 0 {
        GATE = 1;
    }

    // Check array c
    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        GATE = 2;
    }
}

fn check_a(nn: usize, mm: usize, a: &[Vec<Vec<i32>>], p: &[Vec<Vec<i32>>]) -> i32 {
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

fn check_c(nn: usize, mm: usize, c: &[Vec<Vec<Vec<i32>>>], r: &[Vec<Vec<Vec<i32>>>]) -> i32 {
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