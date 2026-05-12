fn check_a(nn: i32, mm: i32, a: &Vec<Vec<Vec<i32>>>, mut p_index: usize) -> i32 {
    for i in 0..nn {
        let p_slice = &a[p_index];
        for j in 0..6 {
            for k in 0..mm {
                if a[i as usize][j][k as usize] != p_slice[j][k as usize] + i {
                    return 1;
                }
            }
        }
        p_index += 1;
    }
    0
}

fn check_c(nn: i32, mm: i32, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i as usize][j as usize][u][v as usize] != r[i as usize][j as usize][u][v as usize] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat(nn: i32, mm: i32, gate: &mut i32) {
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm as usize]; 6]; nn as usize];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm as usize]; 6]; nn as usize]; nn as usize];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i as usize][j][k as usize] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i as usize][j as usize][u][v as usize] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    let p_index = 0;
    if check_a(nn, mm, &a, p_index) != 0 {
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
    fcompat(n, m, &mut gate);

    if gate != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}