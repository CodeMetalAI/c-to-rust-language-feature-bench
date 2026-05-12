static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, mut p_idx: usize) -> i32 {
    for i in 0..nn {
        let p_slice = &a[p_idx];
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != p_slice[j][k] + i as i32 {
                    return 1;
                }
            }
        }
        p_idx += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<i32>>>) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != r[i][j][u] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat(nn: i32, mm: i32) {
    let nn_us = nn as usize;
    let mm_us = mm as usize;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm_us]; 6]; nn_us];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm_us]; 6]; nn_us]; nn_us];

    for i in 0..nn_us {
        for j in 0..6 {
            for k in 0..mm_us {
                a[i][j][k] = ((i + 1) as i32) * 10000 + ((j + 1) as i32) * 100 + ((k + 1) as i32);
            }
        }
    }

    for i in 0..nn_us {
        for j in 0..nn_us {
            for u in 0..6 {
                for v in 0..mm_us {
                    c[i][j][u][v] = ((i + 1) as i32) * 1000000 + ((j + 1) as i32) * 10000 + ((u + 1) as i32) * 100 + ((v + 1) as i32);
                }
            }
        }
    }

    if check_a(nn_us, mm_us, &a, 0) != 0 {
        unsafe { GATE = 1; }
    }

    if check_c(nn_us, mm_us, &c, &c[0]) != 0 {
        unsafe { GATE = 2; }
    }
}

fn main() {
    let n = 6;
    let m = n + 1;

    unsafe { GATE = 0; }
    fcompat(n, m);

    if unsafe { GATE } != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}