static mut N: usize = 0;
static mut M: usize = 0;

static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p_start: usize) -> i32 {
    let mut p = p_start;
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != a[p][j][k] + i as i32 {
                    return 1;
                }
            }
        }
        p += 1;
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

fn fcompat(nn: usize, mm: usize) {
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0i32; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0i32; mm]; 6]; nn]; nn];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = ((i + 1) as i32) * 10000 + ((j + 1) as i32) * 100 + (k + 1) as i32;
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = ((i + 1) as i32) * 1000000 + ((j + 1) as i32) * 10000 + ((u + 1) as i32) * 100 + (v + 1) as i32;
                }
            }
        }
    }

    unsafe {
        if check_a(nn, mm, &a, 0) != 0 {
            GATE = 1;
        }

        if check_c(nn, mm, &c, &c) != 0 {
            GATE = 2;
        }
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;

        GATE = 0;
        fcompat(N, M);

        if GATE != 0 {
            std::process::exit(1);
        }

        std::process::exit(0);
    }
}