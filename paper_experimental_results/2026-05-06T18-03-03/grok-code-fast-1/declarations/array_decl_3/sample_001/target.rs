use std::process;

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut p_idx = 0;
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != a[p_idx][j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_idx += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
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

fn fcompat(nn: usize, mm: usize, gate: &mut i32) {
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
            }
        }
    }
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                }
            }
        }
    }
    if check_a(nn, mm, &a) != 0 {
        *gate = 1;
    }
    if check_c(nn, mm, &c, &c) != 0 {
        *gate = 2;
    }
}

fn main() {
    let n: i32 = 6;
    let m: i32 = n + 1;
    let mut gate: i32 = 0;
    fcompat(n as usize, m as usize, &mut gate);
    if gate != 0 {
        process::exit(1);
    }
    process::exit(0);
}