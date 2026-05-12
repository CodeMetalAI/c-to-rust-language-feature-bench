// array_decl_3.rs
fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, mut p_index: usize) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != a[p_index][j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_index += 1;
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

fn fcompat(nn: i32, mm: i32) -> i32 {
    let nn = nn as usize;
    let mm = mm as usize;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut p_index = 0;

    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];
    let r = &c;

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let mut gate = 0;
    if check_a(nn, mm, &a, p_index) != 0 {
        gate = 1;
    }

    if check_c(nn, mm, &c, r) != 0 {
        gate = 2;
    }

    gate
}

fn main() {
    let n: i32 = 6;
    let m = n + 1;

    let gate = fcompat(n, m);

    if gate != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}