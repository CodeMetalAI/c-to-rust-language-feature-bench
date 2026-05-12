fn check_a(nn: i32, mm: i32, a: &Vec<Vec<Vec<i32>>>, mut p_idx: usize) -> i32 {
    let mut i: i32 = 0;
    while i < nn {
        let p = &a[p_idx];
        let mut j: i32 = 0;
        while j < 6 {
            let mut k: i32 = 0;
            while k < mm {
                if a[i as usize][j as usize][k as usize] != p[j as usize][k as usize] + i {
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

fn check_c(nn: i32, mm: i32, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i: i32 = 0;
    while i < nn {
        let mut j: i32 = 0;
        while j < nn {
            let mut u: i32 = 0;
            while u < 6 {
                let mut v: i32 = 0;
                while v < mm {
                    if c[i as usize][j as usize][u as usize][v as usize]
                        != r[i as usize][j as usize][u as usize][v as usize]
                    {
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

fn fcompat(n: i32, m: i32) -> i32 {
    let nn = n;
    let mm = m;
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm as usize]; 6]; nn as usize];
    for i in 0..nn as usize {
        for j in 0..6 {
            for k in 0..mm as usize {
                a[i][j][k] = ((i + 1) as i32) * 10000 + ((j + 1) as i32) * 100 + ((k + 1) as i32);
            }
        }
    }
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm as usize]; 6]; nn as usize]; nn as usize];
    for i in 0..nn as usize {
        for j in 0..nn as usize {
            for u in 0..6 {
                for v in 0..mm as usize {
                    c[i][j][u][v] = ((i + 1) as i32) * 1000000
                        + ((j + 1) as i32) * 10000
                        + ((u + 1) as i32) * 100
                        + ((v + 1) as i32);
                }
            }
        }
    }
    let p_idx = 0;
    if check_a(nn, mm, &a, p_idx) != 0 {
        return 1;
    }
    if check_c(nn, mm, &c, &c) != 0 {
        return 2;
    }
    0
}

fn main() {
    let n = 6;
    let m = n + 1;
    let gate = fcompat(n, m);
    if gate != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}